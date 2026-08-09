//! Extension for an orthographic matrix
//!

use super::super::vertex_color_shader::camera_uniform::CameraUniform;
use cgmath::{ortho, Matrix4};

impl CameraUniform {
    #[rustfmt::skip]
    pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    pub fn new_orthographic(width: u32, height: u32) -> Self {
        #[rustfmt::skip]
        let mat = ortho(
            0.0,
            width as f32,
            0.0,
            height as f32,
            0.0,
            1.0,
        );

        let mat = Self::OPENGL_TO_WGPU_MATRIX * mat;

        Self {
            view_position: [0.0; 4],
            view_proj: mat.into(),
            proj: mat.into(),
        }
    }

    pub fn resize_orthographic(&mut self, width: u32, height: u32) {
        #[rustfmt::skip]
        let mat = ortho(
            0.0,
            width as f32,
            0.0,
            height as f32,
            0.0,
            1.0,
        );

        let mat = Self::OPENGL_TO_WGPU_MATRIX * mat;

        self.view_proj = mat.into();
    }
}
