//! GPU memory buffer containing the Color for this shader
//!

use super::Color;
use wgpu::util::DeviceExt;

pub struct ColorBuffer {
    buffer: wgpu::Buffer,
}

impl ColorBuffer {
    pub fn new(device: &wgpu::Device, colors: &[Color])  -> Self
    {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Color Buffer"),
                contents: bytemuck::cast_slice(colors),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        Self {
            buffer
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, colors: &[Color])
    {   
        let data = bytemuck::cast_slice(colors);

        if self.buffer.size() == data.len() as u64 {
            queue.write_buffer(&self.buffer, 0, data);
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,) 
    {
        render_pass.set_vertex_buffer(1, self.buffer.slice(..));
    }

}