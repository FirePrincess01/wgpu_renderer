//! Contains the device buffers to render an object with this shader
//!

use super::HeightmapBindGroupLayout;
use super::Vertex;
use super::Heightmap;
use super::Heightmap2D;
use super::Instance;

use super::VertexBuffer;
use super::Texture;
use super::HeightmapTexture;
use super::IndexBuffer;
use super::InstanceBuffer;

/// A general purpose shader using vertices, colors and an instance matrix
pub struct Mesh
{
    vertex_buffer: VertexBuffer,
    texture_index: usize,
    heightmap_texture: HeightmapTexture,
    index_buffer: IndexBuffer,
    instance_buffer: InstanceBuffer,
}

impl Mesh
{
    pub fn new(device: &wgpu::Device, 
        vertices: &[Vertex],
        texture_index: usize,
        heightmap2d: &Heightmap2D,
        heightmap_bind_group_layout: &HeightmapBindGroupLayout,
        indices: &[u32],
        instances: &[Instance]) -> Self
    {
        let vertex_buffer = VertexBuffer::new(device, vertices);
        let heightmap_texture = HeightmapTexture::new(
            device, 
            heightmap_bind_group_layout, 
            heightmap2d.data, 
            heightmap2d.width,
            heightmap2d.height,
            Some("Heightmap Texture"),
        );
        let index_buffer = IndexBuffer::new(device, indices);

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = InstanceBuffer::new(device, &instance_data);

        Self {
            vertex_buffer,
            texture_index,
            heightmap_texture,
            index_buffer,
            instance_buffer,
        }
    }

    pub fn _update_vertex_buffer(&mut self, queue: &wgpu::Queue, vertices: &[Vertex])
    {   
        self.vertex_buffer.update(queue, vertices);
    }

    pub fn _set_texture_index(&mut self, texture_index: usize)
    {
        self.texture_index = texture_index;
    }

    pub fn update_heightmap_texture(&mut self, queue: &wgpu::Queue, heightmap: &[Heightmap])
    {   
        self.heightmap_texture.update(queue, heightmap);
    }

    pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instances: &[Instance])
    {
        let instance_data = &instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        self.instance_buffer.update(queue, instance_data);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, textures: &'a [Texture])
    {
        self.vertex_buffer.bind(render_pass);
        textures[self.texture_index].bind(render_pass);
        self.heightmap_texture.bind(render_pass);
        self.index_buffer.bind(render_pass);
        self.instance_buffer.bind_slot(render_pass, 1);

        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..self.instance_buffer.size());
    }
}