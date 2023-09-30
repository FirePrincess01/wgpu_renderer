//! A bind group to create a heightmap for this shader
//!

pub struct HeightmapBindGroupLayout {
    heightmap_bind_group_layout: wgpu::BindGroupLayout,
}

impl HeightmapBindGroupLayout {

    pub fn new(device: &wgpu::Device) -> Self {

        // Texture
        let heightmap_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                },
            ],
            label: Some("heightmap_bind_group_layout"),
        });

        Self {
            heightmap_bind_group_layout,
        }
    }

    pub fn get(&self) -> &wgpu::BindGroupLayout {
        &self.heightmap_bind_group_layout
    }

}