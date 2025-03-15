//! A simple square

use cgmath::InnerSpace;

use super::{MeshDataInterface, MeshDataPoints, MeshDataTriangles};

pub struct Square {
    mesh_data: MeshDataPoints,
    mesh_data_triangles: MeshDataTriangles,
}

impl Square {
    pub fn new(a: f32) -> Self {
        let positions = [
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(a, 0.0, 0.0),
            cgmath::Vector3::new(0.0, a, 0.0),
            cgmath::Vector3::new(a, a, 0.0),
        ];

        let normal = positions[0].cross(positions[1]).normalize();
        let normals = [normal, normal, normal, normal];

        let indices = [0, 1, 2, 3];

        let mesh_data = MeshDataPoints {
            positions: positions.into(),
            normals: normals.into(),
            indices: indices.into(),
        };

        let mesh_data_triangles = mesh_data.triangulate_grid(2);

        Self {
            mesh_data,
            mesh_data_triangles,
        }
    }
}

impl MeshDataInterface for Square {
    fn points(&self) -> &MeshDataPoints {
        &self.mesh_data
    }

    fn triangles(&self) -> &MeshDataTriangles {
        &self.mesh_data_triangles
    }
}
