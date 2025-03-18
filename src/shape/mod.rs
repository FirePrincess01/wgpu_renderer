//! a collection of shapes

mod square;
mod uv_sphere;
mod grid;

pub use square::Square;
pub use uv_sphere::UVSphere;
pub use grid::Grid;

use cgmath::InnerSpace;

pub trait MeshDataInterface {
    fn points(&self) -> &MeshDataPoints;
    fn triangles(&self) -> &MeshDataTriangles;
}

#[derive(Clone)]
pub struct MeshDataPoints {
    pub positions: Vec<cgmath::Vector3<f32>>,
    pub normals: Vec<cgmath::Vector3<f32>>,
    pub indices: Vec<u32>,
}

#[derive(Clone)]
pub struct MeshDataTriangles {
    pub positions: Vec<cgmath::Vector3<f32>>,
    pub normals: Vec<cgmath::Vector3<f32>>,
    pub indices: Vec<u32>,
}

impl MeshDataPoints {
    pub fn triangulate_grid(&self, n: usize) -> MeshDataTriangles {
        let positions = &self.positions;
        let normals = &self.normals;
        let _indices = &self.indices;

        assert_eq!(positions.len(), n * n);
        assert_eq!(normals.len(), n * n);

        let mut positions_res = Vec::new();
        let mut normals_res = Vec::new();
        let mut indices_res = Vec::new();

        for y in 0..n - 1 {
            for x in 0..n - 1 {
                let pos_0 = positions[y * n + x];
                let pos_1 = positions[y * n + x + 1];
                let pos_2 = positions[(y + 1) * n + x + 1];
                let pos_3 = positions[(y + 1) * n + x];

                let normal_0 = (pos_1 - pos_0).cross(pos_3 - pos_0).normalize();

                let indices_local: [u32; 6] = [
                    0, 1, 2, // triangle 0
                    2, 3, 0, // triangle 1
                ];

                positions_res.push(pos_0);
                positions_res.push(pos_1);
                positions_res.push(pos_2);
                positions_res.push(pos_3);

                normals_res.push(normal_0);
                normals_res.push(normal_0);
                normals_res.push(normal_0);
                normals_res.push(normal_0);

                let current_index = (y * (n - 1) + x) * 4;
                for index in indices_local {
                    indices_res.push(current_index as u32 + index);
                }
            }
        }

        MeshDataTriangles {
            positions: positions_res,
            normals: normals_res,
            indices: indices_res,
        }
    }
}
