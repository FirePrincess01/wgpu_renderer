//! Using an event loop to call functions of a basic wgpu renderer application

use std::sync::Arc;

use winit::{dpi::LogicalSize, window};

use crate::wgpu_renderer::{WgpuRenderer, WgpuRendererInterface};

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
    fn update_scale_factor(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        scale_factor: f32,
    );
    fn update(&mut self, renderer_interface: &mut dyn WgpuRendererInterface, dt: instant::Duration);
    fn input(&mut self, event: &winit::event::WindowEvent) -> bool;
    fn render(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
    ) -> Result<(), wgpu::SurfaceError>;
}

pub struct DefaultApplication<ConcreteApplication: DefaultApplicationInterface> {
    // state
    initial_size: Option<LogicalSize<u32>>,
    window: Option<Arc<winit::window::Window>>,
    wgpu_renderer: Option<WgpuRenderer>,
    app: Option<ConcreteApplication>,

    // Used to send cosutome events to the event loop
    proxy: winit::event_loop::EventLoopProxy<WgpuRenderer>,

    last_render_time: instant::Instant,
}

impl<ConcreteApplication: DefaultApplicationInterface> DefaultApplication<ConcreteApplication> {
    pub fn new(event_loop: &winit::event_loop::EventLoop<WgpuRenderer>) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            console_error_panic_hook::set_once();
            console_log::init_with_level(log::Level::Info).unwrap();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // env_logger::init();
            let mut builder = env_logger::Builder::new();
            builder.target(env_logger::Target::Stdout);
            builder.filter_level(log::LevelFilter::Info);
            builder.write_style(env_logger::WriteStyle::Always);

            builder.init();
        }
        log::info!("Logger initialized");

        let proxy = event_loop.create_proxy();

        Self {
            initial_size: None,
            window: None,
            wgpu_renderer: None,
            app: None,
            last_render_time: instant::Instant::now(),
            proxy,
        }
    }
}

impl<ConcreteApplication: DefaultApplicationInterface>
    winit::application::ApplicationHandler<WgpuRenderer>
    for DefaultApplication<ConcreteApplication>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // create window
        #[allow(unused_mut)]
        let mut window_attributes: window::WindowAttributes = window::Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
            let html_canvas_element: wgpu::web_sys::HtmlCanvasElement = canvas.unchecked_into();
            let width = html_canvas_element.width();
            let height = html_canvas_element.height();
            // log::info!("html_canvas_element {:?}", html_canvas_element);

            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
            self.initial_size = Some(LogicalSize::new(width, height));
        }

        // log::info!("window_attributes {:?}", window_attributes);

        let window = Arc::new(event_loop.create_window(window_attributes.clone()).unwrap());

        self.window = Some(window.clone());
        log::info!("Window created");

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

        // Init wgpu
        let present_mode = Some(wgpu::PresentMode::Immediate);
        let proxy = self.proxy.clone();

        let create_wgpu_renderer = async move {
            let wgpu_renderer = WgpuRenderer::new(window.clone(), present_mode).await;
            log::info!("WgpuRenderer created");

            assert!(proxy.send_event(wgpu_renderer).is_ok())
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to
            // await the
            pollster::block_on(create_wgpu_renderer);
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Run the future asynchronously and use the
            // proxy to send the results to the event loop
            wasm_bindgen_futures::spawn_local(create_wgpu_renderer);
        }
    }

    fn user_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        event: WgpuRenderer,
    ) {
        let window = self.window.as_ref().unwrap();
        let mut wgpu_renderer = event;

        // create app
        let scale_factor = window.scale_factor();
        let size = window.inner_size();
        let mut app = ConcreteApplication::create(&mut wgpu_renderer, size, scale_factor as f32);
        // log::info!("size original: {} {}", size.width, size.height);
        wgpu_renderer.resize(size);
        app.resize(&mut wgpu_renderer, size);

        self.wgpu_renderer = Some(wgpu_renderer);
        self.app = Some(app);
        log::info!("App created");

        self.last_render_time = instant::Instant::now();

        window.request_redraw();
        if let Some(initial_size) = self.initial_size {
            // warkaround for webgl
            let _res = window.request_inner_size(initial_size);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = self.window.as_ref().unwrap();
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.window.is_none() || self.wgpu_renderer.is_none() || self.app.is_none() {
            log::info!("not yet initialized: {:?}", event);
            // event_loop.exit();
            return;
        }

        let window = self.window.as_ref().unwrap();
        let wgpu_renderer = self.wgpu_renderer.as_mut().unwrap();
        let app = self.app.as_mut().unwrap();

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
                    log::info!("rescale: {}", scale_factor);
                    app.update_scale_factor(wgpu_renderer, scale_factor as f32);
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
