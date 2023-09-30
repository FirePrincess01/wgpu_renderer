//! A bind group to create a camera uniform buffer for this shader
//!

pub struct CameraBindGroupLayout {
    camera_bind_group_layout: wgpu::BindGroupLayout,
}

impl CameraBindGroupLayout {

    pub fn new(device: &wgpu::Device) -> Self {

            // Camera
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset:false, 
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        Self {
            camera_bind_group_layout,
        }
    }

    pub fn get(&self) -> &wgpu::BindGroupLayout {
        &self.camera_bind_group_layout
    }

}