//! Contains a buffer for the CameraUniform struct
//!

use super::camera_uniform;
use super::camera_bind_group_layout;
use wgpu::util::DeviceExt;

pub struct CameraUniformBuffer{
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
}

impl CameraUniformBuffer {
    pub fn new(device: &wgpu::Device, camera_bind_group_layout: &camera_bind_group_layout::CameraBindGroupLayout) -> Self {

        let camera_uniform = camera_uniform::CameraUniform::new();

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: camera_bind_group_layout.get(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        Self {
            camera_buffer,
            camera_bind_group,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, camera_uniform: camera_uniform::CameraUniform)
    {
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>){
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
    }

}