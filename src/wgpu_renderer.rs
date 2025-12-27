//! Instantiates the device to render with wgpu
//!

pub mod camera;
pub mod depth_texture;

use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

pub trait WgpuRendererInterface {
    fn device(&mut self) -> &mut wgpu::Device;
    fn queue(&mut self) -> &mut wgpu::Queue;

    fn surface_width(&self) -> u32;
    fn surface_height(&self) -> u32;
    fn surface_format(&self) -> wgpu::TextureFormat;
    fn get_depth_texture_view(&self) -> &wgpu::TextureView;
    fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError>;
    fn enable_vsync(&mut self, enabled: bool);
    fn request_window_size(&mut self, width: u32, height: u32);
}

pub struct WgpuRenderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    depth_texture: depth_texture::DepthTexture,

    window: Arc<Window>,
}

impl WgpuRenderer<'_> {
    pub async fn new(window: Arc<Window>, present_mode: Option<wgpu::PresentMode>) -> Self {
        let present_mode = present_mode.unwrap_or(wgpu::PresentMode::Fifo);

        let size = PhysicalSize {
            width: 800,
            height: 600,
        };

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            backend_options: wgpu::BackendOptions::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds {
                for_resource_creation: None,
                for_device_loss: None,
            },
            // dx12_shader_compiler: Default::default(),
            // gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it
        // State owns the window so this should be safe
        let surface = { instance.create_surface(window.clone()) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // let downlevel_capabilities = adapter.get_downlevel_capabilities();
        // let downlevel_flags = downlevel_capabilities.flags;
        // let vertex_storage = downlevel_flags.contains(wgpu::DownlevelFlags::VERTEX_STORAGE);
        // let compute_shader = downlevel_flags.contains(wgpu::DownlevelFlags::COMPUTE_SHADERS);

        // log::error!("vertex storage: {}", vertex_storage);
        // log::error!("compute shader: {}", compute_shader);

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    let mut defaults = wgpu::Limits::downlevel_webgl2_defaults();
                    defaults.max_texture_dimension_2d = 4096;
                    defaults.max_color_attachment_bytes_per_sample = 64;
                    defaults.max_buffer_size = 1024 << 20; // (1 GiB)
                    defaults
                } else {
                    wgpu::Limits {
                        max_color_attachment_bytes_per_sample: 64,
                        max_buffer_size: 1024 << 20, // (1 GiB)
                        ..Default::default()
                    }
                },
                label: None,
                memory_hints: wgpu::MemoryHints::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account to that when drawing to the frame.
        #[allow(clippy::filter_next)]
        let surface_format = surface_caps
            .formats
            .iter()
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
                if surface_caps.present_modes.contains(&present_mode) {
                    present_mode
                } else {
                    wgpu::PresentMode::Fifo // default, vsync on
                }
            },
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let depth_texture =
            depth_texture::DepthTexture::create_depth_texture(&device, &config, "depth_texture");

        Self {
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,

            window,
        }
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.depth_texture = depth_texture::DepthTexture::create_depth_texture(
                &self.device,
                &self.config,
                "depth_texture",
            );
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

impl WgpuRendererInterface for WgpuRenderer<'_> {
    fn device(&mut self) -> &mut wgpu::Device {
        &mut self.device
    }

    fn queue(&mut self) -> &mut wgpu::Queue {
        &mut self.queue
    }

    fn surface_width(&self) -> u32 {
        self.config.width
    }

    fn surface_height(&self) -> u32 {
        self.config.height
    }

    fn surface_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    fn get_depth_texture_view(&self) -> &wgpu::TextureView {
        &self.depth_texture.view
    }

    fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    fn enable_vsync(&mut self, is_vsync_enabled: bool) {
        if is_vsync_enabled {
            self.config.present_mode = wgpu::PresentMode::AutoVsync;
        } else {
            self.config.present_mode = wgpu::PresentMode::AutoNoVsync;
        }

        self.surface.configure(&self.device, &self.config);
    }

    fn request_window_size(&mut self, width: u32, height: u32) {
        let _res = self
            .window
            .request_inner_size(PhysicalSize::new(width, height));
    }
}
