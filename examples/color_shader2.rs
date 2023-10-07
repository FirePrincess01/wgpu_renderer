
mod geometry;
mod default;

use std::thread;

use cgmath::Point3;
use default::{default_window, default_renderer};
use instant::Duration;
use wgpu_renderer:: {
    renderer,
    vertex_color_shader,
};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, DeviceEvent, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

struct ColorShaderExample 
{
    size: winit::dpi::PhysicalSize<u32>,
    scale_factor: f32,

    // wgpu_renderer
    wgpu_renderer : renderer::WgpuRenderer,
    camera_bind_group_layout: vertex_color_shader::CameraBindGroupLayout,
    pipeline: vertex_color_shader::Pipeline,

    // camera
    camera: renderer::camera::Camera,
    camera_controller: renderer::camera::CameraController,
    projection: renderer::camera::Projection,

    camera_uniform: vertex_color_shader::CameraUniform,
    camera_uniform_buffer: vertex_color_shader::CameraUniformBuffer,

    // Meshes
    circle_mesh: vertex_color_shader::Mesh,
    quad_mesh: vertex_color_shader::Mesh,
}

impl ColorShaderExample {
    pub async fn new(window: &winit::window::Window) -> Self 
    {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let mut wgpu_renderer = renderer::WgpuRenderer::new(window).await;
        let surface_format = wgpu_renderer.config().format;
        let camera_bind_group_layout = vertex_color_shader::CameraBindGroupLayout::new(wgpu_renderer.device());
        let pipeline = vertex_color_shader::Pipeline::new(
            wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            surface_format,
        );

        let position = Point3::new(0.0, 0.0, 0.0);
        let yaw = cgmath::Deg(0.0);
        let pitch = cgmath::Deg(0.0);
        let mut camera = renderer::camera::Camera::new(position, yaw, pitch);
        Self::top_view_point(&mut camera);

        let speed = 1.0;
        let sensitivity = 1.0;
        let camera_controller = renderer::camera::CameraController::new(speed, sensitivity);

        let width = wgpu_renderer.config().width;
        let height = wgpu_renderer.config().height;
        let fovy = cgmath::Deg(45.0);
        let znear = 0.1;
        let zfar = 100.0;
        let projection = renderer::camera::Projection::new(width, height, fovy, znear, zfar);

        let camera_uniform = vertex_color_shader::CameraUniform::new();

        let camera_uniform_buffer = vertex_color_shader::CameraUniformBuffer::new(
            wgpu_renderer.device(), 
            &camera_bind_group_layout);


        // meshes
        let circle = geometry::Circle::new(0.2, 8);
        let quad = geometry::Quad::new(1.0);

        const INSTANCES: &[vertex_color_shader::Instance] = &[ 
            vertex_color_shader::Instance{
                position: glam::Vec3::new(0.0, 0.0, 0.0),
                rotation: glam::Quat::IDENTITY,
            },
        ];

        let circle_mesh = vertex_color_shader::Mesh::new(&mut wgpu_renderer.device(), 
        &circle.vertices, 
        &circle.colors, 
        &circle.indices, 
        &INSTANCES);

        let quad_mesh = vertex_color_shader::Mesh::new(&mut wgpu_renderer.device(), 
        &quad.vertices, 
        &quad.colors, 
        &quad.indices, 
        &INSTANCES);

        Self {
            size,
            scale_factor,
            wgpu_renderer,
            camera_bind_group_layout,
            pipeline,
            camera,
            camera_controller,
            projection,
            camera_uniform,
            camera_uniform_buffer,
            circle_mesh,
            quad_mesh,
        }

    }

    fn top_view_point(camera: &mut renderer::camera::Camera) {
        let position = Point3::new(0.0, 0.0, 67.0*4.0);
        let yaw = cgmath::Deg(-90.0).into();
        let pitch = cgmath::Deg(0.0).into();

        camera.position = position;
        camera.yaw = yaw;
        camera.pitch = pitch;
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        
        thread::sleep(Duration::from_millis(400));

        self.projection.resize(new_size.width, new_size.height);
        self.wgpu_renderer.resize(new_size);

        thread::sleep(Duration::from_millis(400));
    }

    fn update_scale_factor(&mut self, scale_factor: f32) {
        
    }

    fn update(&mut self, dt: instant::Duration) {
        // camera
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        false
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.wgpu_renderer.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.wgpu_renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: Some("Render Pass"), 
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.02,
                            b: 0.03,
                            a: 1.0,
                        }),
                        store: true,
                    }
                })], 
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: self.wgpu_renderer.get_depth_texture_view(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }) 
            });

            self.pipeline.bind(&mut render_pass);
            self.camera_uniform_buffer.bind(&mut render_pass);
            self.circle_mesh.draw(&mut render_pass);
        }

        // self.watch.start(0);
            self.wgpu_renderer.queue().submit(std::iter::once(encoder.finish()));
            output.present();
        // self.watch.stop(0);
        
        Ok(())
    }
}


// runs the event loop
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run()
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
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    use winit::dpi::PhysicalSize;
    window.set_inner_size(PhysicalSize::new(700, 800));

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



    let mut state = ColorShaderExample::new(&window).await;

    let mut last_render_time = instant::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // on the web, the resize event does not fire, so we check the value manually
        #[cfg(target_arch = "wasm32")] 
        {
            if window.inner_size() != state.size
            {
                let scale = window.scale_factor() as f32;
                let size = window.inner_size();
    
                state.update_scale_factor(scale);
                state.resize(size);
            }
        }

        match event {
            // Event::DeviceEvent {
            //     event: DeviceEvent::MouseMotion{ delta, },
            //     .. // We're not using device_id currently
            // } => if state.mouse_pressed() {
            //     state.camera_controller.process_mouse(delta.0, delta.1)
            // },

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !state.input(event) {
                match event {
                    #[cfg(not(target_arch="wasm32"))]
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size} => {
                        state.update_scale_factor(*scale_factor as f32);
                        state.resize(**new_inner_size);
                    }
                    _ => {}
            } 
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                
                state.update(dt);
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    // Err(wgpu::SurfaceError::Lost) => {},
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it
                window.request_redraw();
            }
            _ => {}
        }
    });
}


fn main() {
    println!("Hello from an example!");

    pollster::block_on(run());
}