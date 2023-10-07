

use std::thread;

use instant::Duration;
use wgpu_renderer::renderer::{WgpuRenderer, self};
use wgpu_renderer::vertex_color_shader;

use super::super::geometry;

pub struct Renderer 
{   
    // wgpu_renderer
    wgpu_renderer: WgpuRenderer,
    pipeline_color: vertex_color_shader::Pipeline,

    // camera
    camera: renderer::camera::Camera,
    camera_controller: renderer::camera::CameraController,
    projection: renderer::camera::Projection,

    camera_uniform: vertex_color_shader::CameraUniform,
    camera_uniform_buffer: vertex_color_shader::CameraUniformBuffer,

    camera_uniform_orthographic: vertex_color_shader::CameraUniform,
    camera_uniform_orthographic_buffer: vertex_color_shader::CameraUniformBuffer,

    // meshes
    meshes: Vec<vertex_color_shader::Mesh>,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window) -> Self 
    {   
        // wgpu renderer
        let mut wgpu_renderer = WgpuRenderer::new(window).await; 
        let surface_format = wgpu_renderer.config().format;
        
        // pipeline color
        let camera_bind_group_layout = vertex_color_shader::CameraBindGroupLayout::new(wgpu_renderer.device());
        let pipeline_color = vertex_color_shader::Pipeline::new(
            wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            surface_format,
        );

        // camera 
        let position = cgmath::Point3::new(0.0, 0.0, 0.0);
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

        let camera_uniform_orthographic: vertex_color_shader::CameraUniform = vertex_color_shader::CameraUniform::new_orthographic(width, height);
        let mut camera_uniform_orthographic_buffer = vertex_color_shader::CameraUniformBuffer::new(
                wgpu_renderer.device(), 
                &camera_bind_group_layout);

        camera_uniform_orthographic_buffer.update(wgpu_renderer.queue(), camera_uniform_orthographic);   // add uniform identity matrix

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

        let meshes = vec![quad_mesh, circle_mesh];


        Self {
            wgpu_renderer,

            pipeline_color,

            camera,
            camera_controller,
            projection,

            camera_uniform,
            camera_uniform_buffer,

            camera_uniform_orthographic,
            camera_uniform_orthographic_buffer,

            meshes,
        } 
    }

    fn top_view_point(camera: &mut renderer::camera::Camera) {
        let position = cgmath::Point3::new(0.0, 0.0, 10.0);
        let yaw = cgmath::Deg(-90.0).into();
        let pitch = cgmath::Deg(0.0).into();

        camera.position = position;
        camera.yaw = yaw;
        camera.pitch = pitch;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // self.size = new_size;
        
        self.projection.resize(new_size.width, new_size.height);
        self.wgpu_renderer.resize(new_size);
        
        self.camera_uniform_orthographic.resize_orthographic(new_size.width, new_size.height);
        self.camera_uniform_orthographic_buffer.update(self.wgpu_renderer.queue(), self.camera_uniform_orthographic);
    }

    pub fn update(&mut self, dt: instant::Duration) {

        // camera
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
        self.camera_uniform_buffer.update(self.wgpu_renderer.queue(), self.camera_uniform);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    {
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

            self.pipeline_color.bind(&mut render_pass);
            self.camera_uniform_buffer.bind(&mut render_pass);

            // for mesh in meshes {
            //     // mesh.draw(&mut render_pass);
            // }

            // self.meshes[0].draw(&mut render_pass);
            self.meshes[1].draw(&mut render_pass);

            thread::sleep(Duration::from_millis(20));
        }

        // self.watch.start(0);
            self.wgpu_renderer.queue().submit(std::iter::once(encoder.finish()));
            output.present();
        // self.watch.stop(0);
        
        Ok(())
    }

}

