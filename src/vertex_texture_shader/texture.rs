//! The Texture used in the shader
//!


use super::texture_bind_group_layout::TextureBindGroupLayout;
use super::super::wgpu_renderer;
use anyhow::*;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
}

impl Texture {
    pub fn new(
        wgpu_renderer: &mut impl wgpu_renderer::WgpuRendererInterface,
        texture_bind_group_layout: &TextureBindGroupLayout,
        rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, 
        label: Option<&str>
    ) -> Result<Self> {
        let dimensions = rgba.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = wgpu_renderer.device().create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }
        );

        Self::write_texture(wgpu_renderer.queue(), &texture, &rgba);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = wgpu_renderer.device().create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        let bind_group = wgpu_renderer.device().create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: texture_bind_group_layout.get(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view), // CHANGED!
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler), // CHANGED!
                    }
                ],
                label: Some("texture_bind_group"),
            }
        );

        Ok(Self { texture, view, sampler, bind_group })
    }

    fn write_texture(queue: &wgpu::Queue, texture: &wgpu::Texture, rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>)
    {
        let dimensions = rgba.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row:  Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );
    }

    pub fn write(&self, queue: &wgpu::Queue, rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, ) 
    {
        Self::write_texture(queue, &self.texture, rgba);
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }

}

