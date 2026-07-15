//! Interface to the gpu
//!

pub mod camera;
pub mod depth_texture;
pub mod depth_texture_bind_group_layout;

pub trait WgpuRendererInterface {
    fn device(&mut self) -> &mut wgpu::Device;
    fn queue(&mut self) -> &mut wgpu::Queue;

    fn config(&self) -> &wgpu::SurfaceConfiguration;
    fn surface_width(&self) -> u32;
    fn surface_height(&self) -> u32;
    fn surface_format(&self) -> wgpu::TextureFormat;
    fn get_current_texture(&self) -> wgpu::CurrentSurfaceTexture;
    fn enable_vsync(&mut self, enabled: bool);
    fn request_window_size(&mut self, width: u32, height: u32);
    fn pre_present_notify(&mut self);
}
