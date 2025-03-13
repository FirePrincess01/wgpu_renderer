//! Contains the device buffers to render an object with this shader
//!

use super::Color;
use super::Instance;
use super::InstanceRaw;
use super::Vertex;

use super::ColorBuffer;
use super::IndexBuffer;
use super::InstanceBuffer;
use super::VertexBuffer;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Mesh {
    vertex_buffer: VertexBuffer,
    color_buffer: ColorBuffer,
    index_buffer: IndexBuffer<u32>,
    instance_buffer: InstanceBuffer<InstanceRaw>,
}

#[allow(dead_code)]
impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[Vertex],
        colors: &[Color],
        indices: &[u32],
        instances: &[Instance],
    ) -> Self {
        let vertex_buffer = VertexBuffer::new(device, vertices);
        let color_buffer = ColorBuffer::new(device, colors);
        let index_buffer = IndexBuffer::new(device, indices);

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = InstanceBuffer::new(device, &instance_data);

        Self {
            vertex_buffer,
            color_buffer,
            index_buffer,
            instance_buffer,
        }
    }

    pub fn update_vertex_buffer(&mut self, queue: &wgpu::Queue, vertices: &[Vertex]) {
        self.vertex_buffer.update(queue, vertices);
    }

    pub fn update_color_buffer(&mut self, queue: &wgpu::Queue, colors: &[Color]) {
        self.color_buffer.update(queue, colors);
    }

    pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instances: &[Instance]) {
        let instance_data = &instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        self.instance_buffer.update(queue, instance_data);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.vertex_buffer.bind(render_pass);
        self.color_buffer.bind(render_pass);
        self.index_buffer.bind(render_pass);
        self.instance_buffer.bind_slot(render_pass, 2);

        render_pass.draw_indexed(
            0..self.index_buffer.size(),
            0,
            0..self.instance_buffer.size(),
        );
    }
}
