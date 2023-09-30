//! Creates a texture with a text label

use super::super::wgpu_renderer::WgpuRendererInterface;
use super::super::vertex_texture_shader::{
    Vertex, 
    VertexBuffer, 
    Texture, 
    IndexBuffer, 
    InstanceBuffer, 
    Instance, 
    TextureBindGroupLayout
};

pub struct LabelMesh {
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    texture: Texture,
    instance_buffer: InstanceBuffer,
}

impl LabelMesh {
    pub fn new(wgpu_renderer: &mut impl WgpuRendererInterface, 
        texture_rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        texture_bind_group_layout: &TextureBindGroupLayout,
        instance: &Instance) -> Self
    {
        let width = texture_rgba.width();
        let height = texture_rgba.height();

        let vertex_buffer = VertexBuffer::new(wgpu_renderer.device(), 
            &Self::vertices(width, height));
        let index_buffer = IndexBuffer::new(wgpu_renderer.device(), &Self::indices());

        let texture = Texture::new(
            wgpu_renderer, 
            &texture_bind_group_layout, 
            &texture_rgba, 
            Some("gui texture")).unwrap(); 

        let instance_raw = instance.to_raw();
        let instance_buffer = InstanceBuffer::new(wgpu_renderer.device(), &[instance_raw]);

        Self {
            texture,
            instance_buffer,
            vertex_buffer,
            index_buffer,
        }
    }

    fn vertices(width: u32, height: u32) -> [Vertex; 4]
    {
        let width = width as f32;
        let height = height as f32;

        let vertices: [Vertex; 4] = [
            Vertex { position: [0.0, 0.0, 0.0], tex_coords: [0.0, 1.0] }, // A
            Vertex { position: [width, 0.0, 0.0], tex_coords: [1.0, 1.0] }, // B
            Vertex { position: [width, height, 0.0], tex_coords: [1.0, 0.0] }, // C
            Vertex { position: [0.0, height, 0.0], tex_coords: [0.0, 0.0] }, // D
        ];

        vertices
    }

    fn indices() -> [u32; 6]
    {
        const INDICES: [u32;6] = [
            0, 1, 2,
            2, 3, 0,
        ];

        INDICES
    }

    pub fn update_texture(&mut self, queue: &wgpu::Queue, rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) 
    {
        self.texture.write(queue, rgba)
    }

    pub fn update_instance_buffer(&mut self, queue: &wgpu::Queue, instance: &Instance)
    {
        let instance_raw = instance.to_raw();
        self.instance_buffer.update(queue, &[instance_raw]);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>)
    {
        self.vertex_buffer.bind(render_pass);
        self.texture.bind(render_pass);
        self.index_buffer.bind(render_pass);
        self.instance_buffer.bind_slot(render_pass, 1);

        render_pass.draw_indexed(0..self.index_buffer.size(), 0, 0..self.instance_buffer.size());
    }

}