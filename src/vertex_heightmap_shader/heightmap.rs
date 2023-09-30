//! The Heightmap struct used in the shader
//!


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Heightmap {
    pub height: f32,
}

pub struct Heightmap2D<'a> {
    pub data: &'a [Heightmap],
    pub height: u32,
    pub width: u32,
}

impl Heightmap {
    pub fn _zero() -> Self {
        Self { height: 0.0 }
    }
}