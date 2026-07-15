//! Holds the interface to the gpu and the window

use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use crate::wgpu_renderer::WgpuRendererInterface;

pub struct State {
    pub instance: wgpu::Instance,
    pub window: Arc<winit::window::Window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'static>,
    pub surface_format: wgpu::TextureFormat,
    pub config: wgpu::SurfaceConfiguration,
}

impl State {
    pub async fn new(
        window: Arc<Window>,
        instance: wgpu::Instance,
        surface: wgpu::Surface<'static>,
    ) -> State {
        let present_mode = wgpu::PresentMode::AutoNoVsync;

        let size = PhysicalSize {
            width: 800,
            height: 600,
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    let mut defaults = wgpu::Limits::downlevel_webgl2_defaults();
                    defaults.max_texture_dimension_2d = 4096;
                    // defaults.max_color_attachment_bytes_per_sample = 64;
                    defaults.max_buffer_size = 1024 << 20; // (1 GiB)
                    defaults
                } else {
                    wgpu::Limits {
                        // max_color_attachment_bytes_per_sample: 64,
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
        log::info!("Device created");

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = if surface_caps
            .formats
            .contains(&wgpu::TextureFormat::Rgba8UnormSrgb)
        {
            wgpu::TextureFormat::Rgba8UnormSrgb
        } else if surface_caps
            .formats
            .contains(&wgpu::TextureFormat::Rgba8Unorm)
        {
            wgpu::TextureFormat::Rgba8Unorm
        } else {
            log::info!("surface_caps {:?}", surface_caps);
            panic!("No suitable Texture Format found");
        };

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
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        surface.configure(&device, &config);
        log::info!("Surface configured");

        let state = State {
            instance,
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            config,
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    pub fn get_window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn configure_surface(&self) {
        self.surface.configure(&self.device, &self.config)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config)
        }
    }
}

impl WgpuRendererInterface for State {
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

    fn get_current_texture(&self) -> wgpu::CurrentSurfaceTexture {
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

    fn pre_present_notify(&mut self) {
        self.window.pre_present_notify();
    }

    fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }
}
