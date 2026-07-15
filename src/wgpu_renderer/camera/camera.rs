//! Container for the parameters of the camera
//!

use cgmath::*;

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(self.position, self.get_view_direction(), Vector3::unit_z())
    }

    pub fn get_view_position(&self) -> Vector3<f32> {
        cgmath::Vector3::new(self.position.x, self.position.y, self.position.z)
    }

    pub fn get_view_direction(&self) -> Vector3<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }

    pub fn set_view_direction(&mut self, dir: Vector3<f32>) {
        let dir = dir.normalize();

        self.pitch.0 = dir.y.asin();
        self.yaw.0 = dir.z.atan2(dir.x);
    }
}
