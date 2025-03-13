//! GPU memory buffer containing the instances for this shader
//!

use wgpu::util::DeviceExt;

pub struct InstanceBuffer<TInstance> {
    buffer: wgpu::Buffer,
    _size: u32,
    phantom: std::marker::PhantomData<TInstance>,
}

impl<TInstance> InstanceBuffer<TInstance>
where
    TInstance: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device, instances: &[TInstance]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let _size = instances.len() as u32;

        Self {
            buffer,
            _size,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, instances: &[TInstance]) {
        let data = bytemuck::cast_slice(instances);

        if data.len() as u64 <= self.buffer.size() {
            queue.write_buffer(&self.buffer, 0, data);
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(1, self.buffer.slice(..));
    }

    pub fn bind_slot<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, slot: u32) {
        render_pass.set_vertex_buffer(slot, self.buffer.slice(..));
    }

    pub fn size(&self) -> u32 {
        self._size
    }
}
