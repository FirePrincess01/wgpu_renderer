
mod geometry;
mod default;




use default::{default_window, default_renderer};
use wgpu_renderer:: {
    renderer,
    vertex_color_shader,
};

struct ColorShaderExample 
{
    renderer: default_renderer::Renderer,
}

impl ColorShaderExample {
    pub async fn new(window: &winit::window::Window) -> Self 
    {
        let renderer = default_renderer::Renderer::new(window).await;

        Self {
            renderer,
        }

    }
}


impl default_window::DefaultWindowApp for ColorShaderExample 
{
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    fn update_scale_factor(&mut self, scale_factor: f32) {
        
    }

    fn update(&mut self, dt: instant::Duration) {
        self.renderer.update(dt);
    }

    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        false
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render()
        // Ok(())
    }
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run()
{
    let default_window = default_window::DefaultWindow::new();
    let app = ColorShaderExample::new(&default_window.window).await;

    default_window::run(default_window, app);
}


fn main() {
    println!("Hello from an example!");

    pollster::block_on(run());
}