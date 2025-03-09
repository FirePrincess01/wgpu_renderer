//! Using an event loop to call functions of a basic wgpu renderer application

use std::sync::Arc;

use pollster::FutureExt;
use winit::window;

use crate::renderer::{WgpuRenderer, WgpuRendererInterface};

pub trait DefaultApplicationInterface {
    fn create(
        renderer_interface: &mut dyn WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    ) -> Self;

    fn get_size(&self) -> winit::dpi::PhysicalSize<u32>;
    fn resize(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        new_size: winit::dpi::PhysicalSize<u32>,
    );
    fn update_scale_factor(&mut self, scale_factor: f32);
    fn update(&mut self, renderer_interface: &mut dyn WgpuRendererInterface, dt: instant::Duration);
    fn input(&mut self, event: &winit::event::WindowEvent) -> bool;
    fn render(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
    ) -> Result<(), wgpu::SurfaceError>;
}

pub struct StateApplication<'a, ConcreteApplication: DefaultApplicationInterface> {
    window: Arc<winit::window::Window>,
    wgpu_renderer: WgpuRenderer<'a>,
    app: ConcreteApplication,
}

impl<ConcreteApplication: DefaultApplicationInterface> StateApplication<'_, ConcreteApplication> {
    fn window(&self) -> &window::Window {
        &self.window
    }
}

pub struct DefaultApplication<'a, ConcreteApplication: DefaultApplicationInterface> {
    state: Option<StateApplication<'a, ConcreteApplication>>,

    last_render_time: instant::Instant,
}

impl<ConcreteApplication: DefaultApplicationInterface> Default
    for DefaultApplication<'_, ConcreteApplication>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<ConcreteApplication: DefaultApplicationInterface> DefaultApplication<'_, ConcreteApplication> {
    pub fn new() -> Self {
        Self {
            state: None,
            last_render_time: instant::Instant::now(),
        }
    }
}

impl<ConcreteApplication: DefaultApplicationInterface> winit::application::ApplicationHandler
    for DefaultApplication<'_, ConcreteApplication>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // We need to toggle what logger we are using based on if we are in WASM land or not.
        cfg_if::cfg_if! {

            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
            } else {
                // env_logger::init();
                let mut builder = env_logger::Builder::new();
                builder.target(env_logger::Target::Stdout);
                builder.filter_level(log::LevelFilter::Info);
                builder.write_style(env_logger::WriteStyle::Always);

                builder.init();
            }
        }

        let window = event_loop
            .create_window(winit::window::Window::default_attributes())
            .unwrap();

        let window = Arc::new(window);

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

        // we need to add a canvas to the HTML document that we will host our application
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-demo")?;
                    let canvas = web_sys::Element::from(window.canvas().unwrap());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let mut wgpu_renderer =
            WgpuRenderer::new(window.clone(), Some(wgpu::PresentMode::Immediate)).block_on();
        let size = window.inner_size();
        let scale_factor = window.scale_factor();
        let app = ConcreteApplication::create(&mut wgpu_renderer, size, scale_factor as f32);

        let state = StateApplication {
            window,
            wgpu_renderer,
            app,
        };

        self.state = Some(state);
        self.last_render_time = instant::Instant::now();
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = self.state.as_ref().unwrap().window();
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        let window = state.window.as_ref();
        let wgpu_renderer = &mut state.wgpu_renderer;
        let app = &mut state.app;

        if window.id() == window_id {
            if app.input(&event) {
                // event consumed directly by the application
                return;
            }

            match event {
                winit::event::WindowEvent::ActivationTokenDone {
                    serial: _,
                    token: _,
                } => {}
                winit::event::WindowEvent::Resized(physical_size) => {
                    log::info!("resize: {} {}", physical_size.width, physical_size.height);
                    wgpu_renderer.resize(physical_size);
                    app.resize(wgpu_renderer, physical_size);
                }
                winit::event::WindowEvent::Moved(_physical_position) => {}
                winit::event::WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                winit::event::WindowEvent::Destroyed => {}
                winit::event::WindowEvent::DroppedFile(_path_buf) => {}
                winit::event::WindowEvent::HoveredFile(_path_buf) => {}
                winit::event::WindowEvent::HoveredFileCancelled => {}
                winit::event::WindowEvent::Focused(_) => {}
                winit::event::WindowEvent::KeyboardInput {
                    device_id: _,
                    event:
                        winit::event::KeyEvent {
                            physical_key:
                                winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
                            logical_key: _,
                            text: _,
                            location: _,
                            state: winit::event::ElementState::Pressed,
                            repeat: _,
                            ..
                        },
                    is_synthetic: _,
                } => {
                    event_loop.exit();
                }
                winit::event::WindowEvent::ModifiersChanged(_modifiers) => {}
                winit::event::WindowEvent::Ime(_ime) => {}
                winit::event::WindowEvent::CursorMoved {
                    device_id: _,
                    position: _,
                } => {}
                winit::event::WindowEvent::CursorEntered { device_id: _ } => {}
                winit::event::WindowEvent::CursorLeft { device_id: _ } => {}
                winit::event::WindowEvent::MouseWheel {
                    device_id: _,
                    delta: _,
                    phase: _,
                } => {}
                winit::event::WindowEvent::MouseInput {
                    device_id: _,
                    state: _,
                    button: _,
                } => {}
                winit::event::WindowEvent::PinchGesture {
                    device_id: _,
                    delta: _,
                    phase: _,
                } => {}
                winit::event::WindowEvent::PanGesture {
                    device_id: _,
                    delta: _,
                    phase: _,
                } => {}
                winit::event::WindowEvent::DoubleTapGesture { device_id: _ } => {}
                winit::event::WindowEvent::RotationGesture {
                    device_id: _,
                    delta: _,
                    phase: _,
                } => {}
                winit::event::WindowEvent::TouchpadPressure {
                    device_id: _,
                    pressure: _,
                    stage: _,
                } => {}
                winit::event::WindowEvent::AxisMotion {
                    device_id: _,
                    axis: _,
                    value: _,
                } => {}
                winit::event::WindowEvent::Touch(_touch) => {}
                winit::event::WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    inner_size_writer: _,
                } => {
                    app.update_scale_factor(scale_factor as f32);
                }
                winit::event::WindowEvent::ThemeChanged(_theme) => {}
                winit::event::WindowEvent::Occluded(_) => {}
                winit::event::WindowEvent::RedrawRequested => {
                    let now = instant::Instant::now();
                    let dt = now - self.last_render_time;
                    self.last_render_time = now;

                    app.update(wgpu_renderer, dt);
                    match app.render(wgpu_renderer) {
                        Ok(_) => window.request_redraw(),
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => {
                            let new_size = app.get_size();
                            wgpu_renderer.resize(new_size);
                            app.resize(wgpu_renderer, new_size);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            }
        }

        // event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    }
}
