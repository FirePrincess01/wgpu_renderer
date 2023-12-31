

use crate::{vertex_texture_shader, gui3::{Rectangle, GuiElementInterface, MouseEventResult}, renderer};

use super::utils::{create_rectangle_vertices, create_rectangle_indices};

pub struct GuiTexture<ElementId> 
where ElementId: Copy
{
    mesh: vertex_texture_shader::Mesh,
    instance: vertex_texture_shader::Instance,

    rect: Rectangle<ElementId>,

    // to update the mesh
    has_changed:bool
}

impl<ElementId> GuiTexture<ElementId>  
where ElementId: Copy
{
    pub fn new(wgpu_renderer: &mut impl renderer::WgpuRendererInterface,
        texture_bind_group_layout: &vertex_texture_shader::TextureBindGroupLayout,
        width: u32, 
        height: u32, 
        border: u32, 
        texture_index: usize,
        element_id: ElementId
    ) -> Self 
    {

        // meshes
        let vertices = create_rectangle_vertices(width, height);
        let indices = create_rectangle_indices();
        let instance = vertex_texture_shader::Instance::zero();

        let mesh = vertex_texture_shader::Mesh::new(wgpu_renderer.device(), 
        &vertices, 
        texture_index, 
        &indices, 
        &[instance]);

        let rect: Rectangle<ElementId> = Rectangle::new(element_id, width, height, border);
        
        Self {
            mesh,
            instance,

            rect,

            has_changed: false,
        }
    }

    pub fn set_texture_index(&mut self, texture_index: usize)
    {
        self.mesh.set_texture_index(texture_index);
    }

    pub fn update(&mut self, wgpu_renderer: &mut impl renderer::WgpuRendererInterface)
    {
        if self.has_changed {
            self.has_changed = false;
            self.mesh.update_instance_buffer(wgpu_renderer.queue(), &[self.instance]);
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, textures: &'a [vertex_texture_shader::Texture])
    {
        self.mesh.draw(render_pass, textures);
    }
}

impl<ElementId> GuiElementInterface<ElementId> for GuiTexture<ElementId> 
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