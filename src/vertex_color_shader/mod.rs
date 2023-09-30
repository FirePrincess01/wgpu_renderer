//! A general purpose pipeline using vertices, colors and instances
//!
//! Vertices and Colors are independently updateable
//! The implementation uses wgpu for rendering
//!


pub mod vertex;
pub mod vertex_buffer;
pub mod color;
pub mod color_buffer;
pub mod index_buffer;
pub mod instance;
pub mod instance_buffer;
pub mod mesh;
pub mod pipeline;
pub mod camera_bind_group_layout;
pub mod camera_uniform;
pub mod camera_uniform_buffer;

pub use vertex::Vertex;
pub use vertex_buffer::VertexBuffer;
pub use color::Color;
pub use color_buffer::ColorBuffer;
pub use index_buffer::IndexBuffer;
pub use instance::Instance;
pub use instance::InstanceRaw;
pub use instance_buffer::InstanceBuffer;
pub use mesh::Mesh;
pub use pipeline::Pipeline;
pub use camera_bind_group_layout::CameraBindGroupLayout;
pub use camera_uniform::CameraUniform;
pub use camera_uniform_buffer::CameraUniformBuffer;

