//! A simple Grid

use cgmath::{InnerSpace, Zero};

use super::{MeshDataInterface, MeshDataPoints, MeshDataTriangles};

pub struct Grid {
    mesh_data: MeshDataPoints,
    mesh_data_triangles: MeshDataTriangles,
}

impl Grid {
    pub fn new(a: f32, len: usize) -> Self {
        let size = len * len;

        let mut positions: Vec<cgmath::Vector3<f32>> = Vec::with_capacity(size);
        let mut normals: Vec<cgmath::Vector3<f32>> = Vec::with_capacity(size);
        let mut indices: Vec<u32> = Vec::with_capacity(size);
        for y in 0 ..len {
            for x in 0 .. len {
                positions.push(cgmath::Vector3 { x: x as f32 * a, y: y as f32 * a, z: 0.0 });
            }
        }

        normals.resize(size, 
            cgmath::Vector3::zero());


        for i in 0..size {
            indices.push(i as u32);
        }

        let mesh_data = MeshDataPoints {
            positions,
            normals,
            indices,
        };

        let mesh_data_triangles = mesh_data.triangulate_grid(len);

        Self {
            mesh_data,
            mesh_data_triangles,
        }
    }
}

impl MeshDataInterface for Grid {
    fn points(&self) -> &MeshDataPoints {
        &self.mesh_data
    }

    fn triangles(&self) -> &MeshDataTriangles {
        &self.mesh_data_triangles
    }
}
