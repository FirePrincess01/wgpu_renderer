//! GPU memory buffer containing the vertices for this shader
//!

use wgpu::util::DeviceExt;

pub struct VertexBuffer<TVertex>
where
    TVertex: bytemuck::Pod,
{
    buffer: wgpu::Buffer,
    phantom: std::marker::PhantomData<TVertex>,
}

impl<TVertex> VertexBuffer<TVertex>
where
    TVertex: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device, vertices: &[TVertex]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, vertices: &[TVertex]) {
        let data = bytemuck::cast_slice(vertices);

        if self.buffer.size() == data.len() as u64 {
            queue.write_buffer(&self.buffer, 0, data);
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
    }
}
