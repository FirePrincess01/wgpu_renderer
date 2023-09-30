//! The Color struct used in the shader
//!

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub color: [f32; 3],
}

impl Color {
    pub fn _black() -> Self {
        Self { color: [0.0, 0.0, 0.0] }
    }

    pub fn white() -> Self {
        Self { color: [1.0, 1.0, 1.0] }
    }
 
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Color>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3
                }
            ]
        }
    }
}