//! The Texture used in the shader
//!


use super::texture_bind_group_layout::TextureBindGroupLayout;
use super::super::renderer;
use anyhow::*;
use image::Rgba;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
    nr_mipmaps: u32,
}

impl Texture {
    pub fn new(
        wgpu_renderer: &mut impl renderer::WgpuRendererInterface,
        texture_bind_group_layout: &TextureBindGroupLayout,
        rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, 
        label: Option<&str>
    ) -> Result<Self> {
        Self::new_with_mipmaps(wgpu_renderer, texture_bind_group_layout, rgba, label, 1)
    }

    pub fn new_with_mipmaps(
        wgpu_renderer: &mut impl renderer::WgpuRendererInterface,
        texture_bind_group_layout: &TextureBindGroupLayout,
        rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, 
        label: Option<&str>,
        nr_mipmaps: u32,
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
                mip_level_count: nr_mipmaps,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }
        );

        Self::write_texture(wgpu_renderer.queue(), &texture, &rgba, nr_mipmaps);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = wgpu_renderer.device().create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
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
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("texture_bind_group"),
            }
        );

        Ok(Self { texture, view, sampler, bind_group, nr_mipmaps })
    }

    fn write_texture(queue: &wgpu::Queue, texture: &wgpu::Texture, rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, nr_mipmaps: u32)
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
        if nr_mipmaps == 1 {return;}

        let mut small_image = Self::downsample(rgba);

        
        for i in 1..nr_mipmaps {
            let dimensions = small_image.dimensions();
            let size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: texture,
                    mip_level: i,
                    origin: wgpu::Origin3d::ZERO,
                },
                &small_image,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row:  Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                size,
            );

            if i != nr_mipmaps - 1 {
                small_image = Self::downsample(&small_image);
            }
        }
    }

    fn downsample(rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let mut small_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(rgba.width()/2, rgba.height()/2);
        for x in 0..small_image.width() {
            for y in 0..small_image.height() {
                let p1: Rgba<u8> = *rgba.get_pixel(2 * x, 2 * y);
                //let p2: Rgba<u8> = *rgba.get_pixel(2 * x + 1, 2 * y);
                //let p3: Rgba<u8> = *rgba.get_pixel(2 * x, 2 * y + 1);
                //let p4: Rgba<u8> = *rgba.get_pixel(2 * x + 1, 2 * y + 1);
                // let averaged_pixel: Rgba<u8> = Rgba([
                //     ((p1[0] as f32 * p1[0] as f32 + p2[0] as f32 * p2[0] as f32 + p3[0] as f32 * p3[0] as f32 + p4[0] as f32 * p4[0] as f32).sqrt() / 2.) as u8,
                //     ((p1[1] as f32 * p1[1] as f32 + p2[1] as f32 * p2[1] as f32 + p3[1] as f32 * p3[1] as f32 + p4[1] as f32 * p4[1] as f32).sqrt() / 2.) as u8,
                //     ((p1[2] as f32 * p1[2] as f32 + p2[2] as f32 * p2[2] as f32 + p3[2] as f32 * p3[2] as f32 + p4[2] as f32 * p4[2] as f32).sqrt() / 2.) as u8,
                //     ((p1[3] as f32 * p1[3] as f32 + p2[3] as f32 * p2[3] as f32 + p3[3] as f32 * p3[3] as f32 + p4[3] as f32 * p4[3] as f32).sqrt() / 2.) as u8
                //     ]);
                small_image.put_pixel(x, y, p1);//averaged_pixel);
            }
        }
        small_image
    }


    pub fn write(&self, queue: &wgpu::Queue, rgba: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, ) 
    {
        Self::write_texture(queue, &self.texture, rgba, self.nr_mipmaps);
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>,) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }

}

