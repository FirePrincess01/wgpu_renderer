

use crate::{vertex_texture_shader, gui3::{Rectangle, GuiElementInterface, MouseEventResult}, renderer, label};

use super::utils::{create_rectangle_vertices, create_rectangle_indices, create_texture_rgba};

pub struct GuiLabel<ElementId> 
where ElementId: Copy
{
    // host data
    label: label::Label,
    instance: vertex_texture_shader::Instance,

    // device data
    mesh: vertex_texture_shader::Mesh,
    textures: [vertex_texture_shader::Texture; 1],

    // placement
    rect: Rectangle<ElementId>,

    // to update the device data
    has_changed:bool
}

impl<ElementId> GuiLabel<ElementId>  
where ElementId: Copy
{
    pub fn new(wgpu_renderer: &mut impl renderer::WgpuRendererInterface,
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        font: &rusttype::Font,
        text: &str,
        scale: u32, 
        border: u32, 
        element_id: ElementId
    ) -> Self 
    {

        // meshes
        let label = label::Label::new(
            &font, scale as f32, text
        );

        let vertices = create_rectangle_vertices(label.width(), label.height());
        let indices = create_rectangle_indices();
        let instance = vertex_texture_shader::Instance::zero();

        let mesh = vertex_texture_shader::Mesh::new(wgpu_renderer.device(), 
        &vertices, 
        0, 
        &indices, 
        &[instance]);

        let textures = [create_texture_rgba(wgpu_renderer, &texture_bind_group_layout, label.get_image())];

        let rect: Rectangle<ElementId> = Rectangle::new(element_id, label.width(), label.height(), border);
        
        Self {
            label,
            instance,

            mesh,
            textures,

            rect,

            has_changed: false,
        }
    }

    pub fn set_text<'a>(&mut self, 
        wgpu_renderer: &mut impl renderer::WgpuRendererInterface, 
        font: &'a rusttype::Font, 
        text: &str) 
    {
        self.label.update(font, &text);
        self.textures[0].write(wgpu_renderer.queue(), self.label.get_image());
    }

    pub fn update(&mut self, wgpu_renderer: &mut impl renderer::WgpuRendererInterface)
    {
        if self.has_changed {
            self.has_changed = false;
            self.mesh.update_instance_buffer(wgpu_renderer.queue(), &[self.instance]);
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>)
    {
        self.mesh.draw(render_pass, &self.textures);
    }
}

impl<ElementId> GuiElementInterface<ElementId> for GuiLabel<ElementId> 
where ElementId: Copy
{
    fn width(&self) -> u32 {
        self.rect.width()
    }

    fn height(&self) -> u32 {
        self.rect.height()
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32) {
        self.instance.position.x = (abs_x + self.rect.boarder()) as f32;
        self.instance.position.y = (abs_y + self.rect.boarder()) as f32;
    
        self.rect.resize(abs_x, abs_y);

        self.has_changed = true;
    }

    fn mouse_event(&mut self, abs_x: u32, abs_y: u32, pressed: bool, res: &mut MouseEventResult<ElementId>) {
        self.rect.mouse_event(abs_x, abs_y, pressed, res)
    }
}