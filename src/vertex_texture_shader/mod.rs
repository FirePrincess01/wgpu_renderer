//! A general purpose pipeline using vertices, textures and instances
//!

mod pipeline;
mod mesh;

mod vertex;
mod vertex_buffer;
mod texture_bind_group_layout;
mod texture;

pub use pipeline::Pipeline;
pub use mesh::Mesh;

pub use vertex::Vertex;
pub use vertex_buffer::VertexBuffer;
pub use texture_bind_group_layout::TextureBindGroupLayout;
pub use texture::Texture;

pub use super::vertex_color_shader::IndexBuffer;
pub use super::vertex_color_shader::Instance;
pub use super::vertex_color_shader::InstanceRaw;
pub use super::vertex_color_shader::InstanceBuffer;

pub use super::vertex_color_shader::CameraBindGroupLayout;
pub use super::vertex_color_shader::CameraUniform;
pub use super::vertex_color_shader::CameraUniformBuffer;