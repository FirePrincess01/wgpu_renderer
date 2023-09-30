//! The HeightmapTexture used in the shader
//!


use super::{HeightmapBindGroupLayout, Heightmap};

pub struct HeightmapTexture {
    pub texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

impl HeightmapTexture {
    pub fn new(
        device: &wgpu::Device,
        heightmap_bind_group_layout: &HeightmapBindGroupLayout,
        _heightmap: &[Heightmap], 
        width: u32,
        height: u32,
        label: Option<&str>
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R32Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: heightmap_bind_group_layout.get(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view), // CHANGED!
                    },
                ],
                label: Some("texture_bind_group"),
            }
        );

        Self { 
            texture, 
            bind_group,
            width,
            height,
        }
    }

    pub fn update(&self, queue: &wgpu::Queue, heightmap: &[Heightmap], ) 
    {
        let size = wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        };

        let data = bytemuck::cast_slice(heightmap);

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row:  Some(4 * self.width),
                rows_per_image: Some(self.height,),
            },
            size,
        );
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,) {
        render_pass.set_bind_group(2, &self.bind_group, &[]);
    }

}

