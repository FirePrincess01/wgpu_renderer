//! Container an description of a texture for wgpu
//!

use crate::wgpu_renderer::{
    depth_texture_bind_group_layout::DepthTextureBindGroupLayout, WgpuRendererInterface,
};

pub struct DepthTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    // pub sampler_filter: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
}

impl DepthTexture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        depth_texture_bind_group_layout: &DepthTextureBindGroupLayout,
        label: &str,
    ) -> Self {
        let config = wgpu_renderer.config();

        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: Default::default(),
        };
        let texture = wgpu_renderer.device().create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = wgpu_renderer
            .device()
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::MipmapFilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            });

        // let sampler_filter = wgpu_renderer.device().create_sampler(&wgpu::SamplerDescriptor {
        //     address_mode_u: wgpu::AddressMode::ClampToEdge,
        //     address_mode_v: wgpu::AddressMode::ClampToEdge,
        //     address_mode_w: wgpu::AddressMode::ClampToEdge,
        //     mag_filter: wgpu::FilterMode::Linear,
        //     min_filter: wgpu::FilterMode::Linear,
        //     mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        //     compare: None,
        //     lod_min_clamp: 0.0,
        //     lod_max_clamp: 100.0,
        //     ..Default::default()
        // });

        let bind_group = wgpu_renderer
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: depth_texture_bind_group_layout.get(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: Some("depth_texture_bind_group"),
            });

        Self {
            texture,
            view,
            sampler,
            // sampler_filter,
            bind_group,
        }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(3, &self.bind_group, &[]);
    }
}
