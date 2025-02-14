//! A general purpose pipeline using vertices, textures and instances
//!

mod mesh;
mod pipeline;

mod texture;
mod texture_bind_group_layout;
mod vertex;
mod vertex_buffer;
mod vertex_texture_shader_draw;

pub use mesh::Mesh;
pub use pipeline::Pipeline;

pub use texture::Texture;
pub use texture_bind_group_layout::TextureBindGroupLayout;
pub use vertex::Vertex;
pub use vertex_buffer::VertexBuffer;

pub use vertex_texture_shader_draw::VertexTextureShaderDraw;

pub use super::vertex_color_shader::IndexBuffer;
pub use super::vertex_color_shader::Instance;
pub use super::vertex_color_shader::InstanceBuffer;
pub use super::vertex_color_shader::InstanceRaw;

pub use super::vertex_color_shader::CameraBindGroupLayout;
pub use super::vertex_color_shader::CameraUniform;
pub use super::vertex_color_shader::CameraUniformBuffer;
