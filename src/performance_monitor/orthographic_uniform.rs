//! Extension for an orthographic matrix
//!

use super::super::vertex_color_shader::camera_uniform::CameraUniform;

impl CameraUniform {
    pub fn new_orthographic(width: u32, height: u32) -> Self {

        let mat = glam::Mat4::orthographic_rh(0.0, width as f32, 0.0, height as f32, 0.0, 1.0);
        let proj: [[f32; 4]; 4] = mat.to_cols_array_2d();
        Self {
            view_position: [0.0; 4],
            view_proj: proj,
        }
    }

    pub fn resize_orthographic(&mut self, width: u32, height: u32) {
        let mat = glam::Mat4::orthographic_rh(0.0, width as f32, 0.0, height as f32, 0.0, 1.0);
        let proj: [[f32; 4]; 4] = mat.to_cols_array_2d();
        self.view_proj = proj;
    }

}