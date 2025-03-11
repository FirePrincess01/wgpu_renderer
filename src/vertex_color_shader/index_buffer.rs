//! GPU memory buffer containing the indices for this shader
//!

use wgpu::util::DeviceExt;

pub struct IndexBuffer<T>
where
    T: bytemuck::Pod,
{
    buffer: wgpu::Buffer,
    size: u32,
    phantom: std::marker::PhantomData<T>,
}

impl<T> IndexBuffer<T>
where
    T: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device, indices: &[T]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let size = indices.len() as u32;

        Self {
            buffer,
            size,
            phantom: std::marker::PhantomData,
        }
    }

    // pub fn update(&mut self, queue: &mut wgpu::Queue, indices: &[u32])
    // {
    //     let data = bytemuck::cast_slice(&indices);

    //     if self.buffer.size() == data.len() as u64 {
    //         queue.write_buffer(&self.buffer, 0, data);
    //     }
    // }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl IndexBuffer<u32> {
    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_index_buffer(self.buffer.slice(..), wgpu::IndexFormat::Uint32);
    }
}

impl IndexBuffer<u16> {
    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_index_buffer(self.buffer.slice(..), wgpu::IndexFormat::Uint16);
    }
}
