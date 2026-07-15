//! A bind group to use the depth texture
//!

pub struct DepthTextureBindGroupLayout {
    shadow_map_bind_group_layout: wgpu::BindGroupLayout,
}

impl DepthTextureBindGroupLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        // Texture
        let shadow_map_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Depth,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                        // ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), // does not work with wgsl
                        count: None,
                    },
                ],
                label: Some("depth_texture_bind_group_layout"),
            });

        Self {
            shadow_map_bind_group_layout,
        }
    }

    pub fn get(&self) -> &wgpu::BindGroupLayout {
        &self.shadow_map_bind_group_layout
    }
}
