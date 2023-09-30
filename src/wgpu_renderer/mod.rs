//! Instantiates the device to render with wgpu
//!

pub mod camera;
pub mod depth_texture;

use winit::window::Window;

// use log::{info, trace, warn, error};

pub trait WgpuRendererInterface{
    fn device(&mut self) -> &mut wgpu::Device;
    fn queue(&mut self) -> &mut wgpu::Queue;
}

pub struct WgpuRenderer
{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    depth_texture: depth_texture::DepthTexture,
}

impl WgpuRenderer
{
    pub async fn new(window: &Window) -> Self 
    {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it
        // State owns the window so this should be safe
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        // let downlevel_capabilities = adapter.get_downlevel_capabilities();
        // let downlevel_flags = downlevel_capabilities.flags;
        // let vertex_storage = downlevel_flags.contains(wgpu::DownlevelFlags::VERTEX_STORAGE);
        // let compute_shader = downlevel_flags.contains(wgpu::DownlevelFlags::COMPUTE_SHADERS);

        // log::error!("vertex storage: {}", vertex_storage);
        // log::error!("compute shader: {}", compute_shader);

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    let mut defaults = wgpu::Limits::downlevel_webgl2_defaults();
                    defaults.max_texture_dimension_2d = 4096;
                    defaults
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, 
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account fo that when drawing to the frame.
        #[allow(clippy::filter_next)]
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: {
                if surface_caps.present_modes.len() >= 2 {
                    surface_caps.present_modes[0]  // vsync on
                    // surface_caps.present_modes[1]  // vsync off
                }
                else {
                    surface_caps.present_modes[0]
                }
            },           
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![]
        };

        surface.configure(&device, &config);

        let depth_texture = depth_texture::DepthTexture::create_depth_texture(&device, &config, "depth_texture");

        Self {
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.depth_texture = depth_texture::DepthTexture::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.surface.configure(&self.device, &self.config)
        }
    }

    pub fn device(&mut self) -> &mut wgpu::Device {
        &mut self.device
    }

    pub fn queue(&mut self) -> &mut wgpu::Queue {
        &mut self.queue
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        
        self.surface.get_current_texture()
    }

    pub fn get_depth_texture_view(&self) -> &wgpu::TextureView {
        &self.depth_texture.view
    }
}

impl WgpuRendererInterface for WgpuRenderer 
{
    fn device(&mut self) -> &mut wgpu::Device {
        &mut self.device
    }

    fn queue(&mut self) -> &mut wgpu::Queue {
        &mut self.queue
    }
}