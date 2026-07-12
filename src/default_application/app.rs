//! Handles the event loop and calls the user application

use std::sync::Arc;

use crate::default_application::{state::State, user_app_builder::UserAppBuilderInterface};

pub struct App {
    /// Taken on the first `resumed` call so initialization happens once.
    pub proxy: Option<winit::event_loop::EventLoopProxy<UserEvent>>,
    pub state: Option<State>,
    pub user_app: Box<dyn UserAppBuilderInterface>,

    pub last_render_time: instant::Instant,
    pub initial_size: Option<winit::dpi::LogicalSize<u32>>,
}

impl winit::application::ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(proxy) = self.proxy.take() else {
            return;
        };

        #[allow(unused_mut)]
        let mut window_attributes: winit::window::WindowAttributes =
            winit::window::Window::default_attributes();

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
            self.initial_size = Some(winit::dpi::LogicalSize::new(width, height));
        }

        // log::info!("window_attributes {:?}", window_attributes);

        let window = Arc::new(event_loop.create_window(window_attributes.clone()).unwrap());

        // Create the surface here, on the main thread: winit only hands out the
        // raw window handle from the thread that owns the window (on Windows,
        // `window_handle()` errors with `Unavailable` anywhere else).
        //
        // `InstanceDescriptor::new_without_display_handle_from_env` honors
        // `WGPU_BACKEND` (e.g. `vulkan`, `dx12`) so each backend's color-space
        // path can be exercised separately.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(event_loop.owned_display_handle()),
        ));
        let surface = instance.create_surface(window.clone()).unwrap();

        // The async tail (adapter, device, pipeline) runs off the main thread
        // (native) or in the browser event loop (web); the finished `State`
        // arrives back on the main thread via `user_event`.
        spawn(async move {
            let state = State::new(window, instance, surface).await;
            let _ = proxy.send_event(UserEvent::Initialized(Box::new(state)));
        });
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::Initialized(state) => {
                let mut state = *state;

                let scale_factor = state.window.scale_factor();
                let size = state.window.inner_size();
                self.user_app.create(&mut state, size, scale_factor as f32);
                log::info!("User App created");

                self.last_render_time = instant::Instant::now();

                state.window.request_redraw();

                if let Some(initial_size) = self.initial_size {
                    // warkaround for webgl
                    let _res = state.window.request_inner_size(initial_size);
                }

                self.state = Some(state);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            log::info!("state not yet initialized: {:?}", event);
            return;
        };

        let Some(user_app) = self.user_app.get() else {
            log::info!("user_app not yet initialized: {:?}", event);
            return;
        };

        if state.window.id() == window_id {
            if user_app.input(&event) {
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
                    state.resize(physical_size);
                    user_app.resize(state, physical_size);
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
                    user_app.update_scale_factor(state, scale_factor as f32);
                }
                winit::event::WindowEvent::ThemeChanged(_theme) => {}
                winit::event::WindowEvent::Occluded(_) => {}
                winit::event::WindowEvent::RedrawRequested => {
                    let now = instant::Instant::now();
                    let dt = now - self.last_render_time;
                    self.last_render_time = now;

                    user_app.update(state, dt);
                    match user_app.render(state) {
                        Ok(_) => state.window.request_redraw(),
                        // Reconfigure the surface if lost
                        Err(_) => {
                            let new_size = user_app.get_size();
                            state.resize(new_size);
                            user_app.resize(state, new_size);
                        } // Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                          // Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            }
        }
    }

    // fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
    //     //  log::info!("about_to_wait");
    //     if let Some(state) = &self.state {
    //         state.window.request_redraw();
    //     }
    // }
}

/// Events delivered to the winit loop from outside a `WindowEvent`.
pub enum UserEvent {
    /// The async setup finished; carries the initialized `State`. Boxed to keep
    /// the event small (`State` is large).
    Initialized(Box<State>),
}

/// Run a future to completion concurrently: on a worker thread on native, or in
/// the browser's event loop on the web (where blocking is not allowed).
#[cfg(not(target_arch = "wasm32"))]
fn spawn(future: impl core::future::Future<Output = ()> + Send + 'static) {
    std::thread::spawn(move || pollster::block_on(future));
}

/// Run a future to completion concurrently: on a worker thread on native, or in
/// the browser's event loop on the web (where blocking is not allowed).
#[cfg(target_arch = "wasm32")]
fn spawn(future: impl core::future::Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(future);
}
