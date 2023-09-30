//! The CameraUniform struct used in the shader
//!

use super::super::wgpu_renderer;
use cgmath::prelude::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_position: [f32; 4],

    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        // use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    // fn update_view_proj(&mut self, camera: &Camera) {
    //     self.view_proj = camera.build_view_projection_matrix().into();
    // }

    pub fn update_view_proj(&mut self, 
        camera: &wgpu_renderer::camera::Camera, 
        projection: &wgpu_renderer::camera::Projection) 
    {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}