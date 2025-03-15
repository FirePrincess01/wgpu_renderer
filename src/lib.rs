// #![deny(unused_crate_dependencies)]

#[cfg(feature = "render")]
pub mod default_application;
#[cfg(feature = "render")]
pub mod freefont;
#[cfg(feature = "render")]
pub mod gui;
#[cfg(feature = "render")]
pub mod label;
#[cfg(feature = "watch")]
pub mod performance_monitor;
#[cfg(feature = "render")]
pub mod shape;
#[cfg(feature = "render")]
pub mod vertex_color_shader;
#[cfg(feature = "render")]
pub mod vertex_heightmap_shader;
#[cfg(feature = "render")]
pub mod vertex_texture_shader;
#[cfg(feature = "render")]
pub mod wgpu_renderer;
