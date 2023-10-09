//! An implementation for a window with an event loop


pub trait DefaultWindowApp
{
    fn get_size(&self) -> winit::dpi::PhysicalSize<u32>;
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn update_scale_factor(&mut self, scale_factor: f32);
    fn update(&mut self, dt: instant::Duration);
    fn input(&mut self, event: &winit::event::WindowEvent) -> bool;
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}

pub struct DefaultWindow
{
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub window: winit::window::Window,
}

impl DefaultWindow {
    pub fn new() -> Self
    {
        // We need to toggle what logger we are using based on if we are in WASM land or not. 
        cfg_if::cfg_if! {
            
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }

        // create our event loop and window
        let event_loop: winit::event_loop::EventLoop<()> = winit::event_loop::EventLoop::new();
        let window: winit::window::Window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

        use winit::dpi::PhysicalSize;
        let size = PhysicalSize::new(700, 800);
        window.set_inner_size(size);

        // wait for the window to resize
        while window.inner_size() != size {}

        // we need to add a canvas to the HTML document that we will host our application
        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            // use winit::dpi::PhysicalSize;
            // window.set_inner_size(PhysicalSize::new(600, 800));
            
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-demo")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        Self {
            event_loop,
            window,
        }
    }    
}

pub fn run<T:DefaultWindowApp + 'static>(default_window: DefaultWindow, app: T)
    {
        let mut state = app;
        let window = default_window.window;
        let event_loop = default_window.event_loop;
    
        let mut last_render_time = instant::Instant::now();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;
    
            // on the web, the resize event does not fire, so we check the value manually
            #[cfg(target_arch = "wasm32")] 
            {
                if window.inner_size() != state.get_size()
                {
                    let scale = window.scale_factor() as f32;
                    let size = window.inner_size();
        
                    state.update_scale_factor(scale);
                    state.resize(size);
                }
            }
    
            match event {
                // winit::event::Event::DeviceEvent {
                //     event: winit::event::DeviceEvent::MouseMotion{ delta, },
                //     .. // We're not using device_id currently
                // } => if state.mouse_pressed() {
                //     state.camera_controller.process_mouse(delta.0, delta.1)
                // },
    
                winit::event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => if !state.input(event) {
                    match event {
                        #[cfg(not(target_arch="wasm32"))]
                        winit::event::WindowEvent::CloseRequested
                        | winit::event::WindowEvent::KeyboardInput {
                            input:
                                winit::event::KeyboardInput {
                                    state: winit::event::ElementState::Pressed,
                                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = winit::event_loop::ControlFlow::Exit,
                        winit::event::WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        winit::event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size} => {
                            state.update_scale_factor(*scale_factor as f32);
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                } 
                },
                winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                    let now = instant::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    
                    state.update(dt);
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        // Err(wgpu::SurfaceError::Lost) => { *control_flow = winit::event_loop::ControlFlow::Exit; },
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.get_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = winit::event_loop::ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                winit::event::Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually request it
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }