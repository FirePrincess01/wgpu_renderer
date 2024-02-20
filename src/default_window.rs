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

        // create our event loop and window
        let event_loop: winit::event_loop::EventLoop<()> = winit::event_loop::EventLoop::new().unwrap();
        let window_builder = winit::window::WindowBuilder::new();
        let window: winit::window::Window = window_builder.build(&event_loop).unwrap();

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

        Self {
            event_loop,
            window,
        }
    }    
}

pub fn run<'a>(event_loop: winit::event_loop::EventLoop<()>, window: &'a winit::window::Window, app: impl DefaultWindowApp)
    {
        let mut state = app;
    
        let mut last_render_time = instant::Instant::now();
        
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

        let res: Result<(), winit::error::EventLoopError> = event_loop.run(move |event, control_flow| {   
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
                            event:
                            winit::event::KeyEvent {
                                    state: winit::event::ElementState::Pressed,
                                    physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        winit::event::WindowEvent::Resized(physical_size) => {
                            log::info!("resize: {} {}", physical_size.width, physical_size.height);
                            state.resize(*physical_size);
                        }
                        winit::event::WindowEvent::ScaleFactorChanged { 
                            scale_factor, 
                            ..
                        } => {
                            state.update_scale_factor(*scale_factor as f32);
                        },
                        winit::event::WindowEvent::RedrawRequested =>  {
                            let now = instant::Instant::now();
                            let dt = now - last_render_time;
                            last_render_time = now;
                            
                            state.update(dt);
                            match state.render() {
                                Ok(_) => { window.request_redraw(); }
                                // Reconfigure the surface if lost
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.get_size()),
                                Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }

                            
                        },
                        _ => {}
                } 
                },
                _ => {}
            }
        });

        res.unwrap();
    }