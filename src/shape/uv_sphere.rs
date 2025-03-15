use cgmath::{Angle, Zero};

use super::{MeshDataInterface, MeshDataPoints, MeshDataTriangles};

pub struct UVSphere {
    mesh_data: MeshDataPoints,
    mesh_data_triangles: MeshDataTriangles,
}

impl UVSphere {
    pub fn new(radius: f32, n: usize) -> Self {
        let mut grid: Vec<cgmath::Vector3<f32>> = Vec::new();
        grid.resize(n * n, cgmath::Vector3::zero());

        for j in 0..n {
            let alpha = -90.0 + 180.0 / (n - 1) as f32 * j as f32;
            let current_radius = radius * cgmath::Deg(alpha).cos();
            let z = radius * cgmath::Deg(alpha).sin();

            for i in 0..n {
                let beta = 360.0 / (n - 1) as f32 * i as f32;

                let x = current_radius * cgmath::Deg(beta).cos();
                let y = current_radius * cgmath::Deg(beta).sin();

                grid[j * n + i] = cgmath::Vector3::new(x, y, z);
            }
        }

        // let positions: Vec<cgmath::Vector3<f32>> = Vec::new();
        // let normals: Vec<cgmath::Vector3<f32>> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let positions = grid.clone();
        let normals = grid;

        for i in 0..n * n {
            indices.push(i as u16);
        }

        let mesh_data = MeshDataPoints {
            positions,
            normals,
            indices,
        };

        let mesh_data_triangles = mesh_data.triangulate_grid(n);

        Self {
            mesh_data,
            mesh_data_triangles,
        }
    }
}

impl MeshDataInterface for UVSphere {
    fn points(&self) -> &MeshDataPoints {
        &self.mesh_data
    }

    fn triangles(&self) -> &MeshDataTriangles {
        &self.mesh_data_triangles
    }
}
