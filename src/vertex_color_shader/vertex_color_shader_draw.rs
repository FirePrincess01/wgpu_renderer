pub trait VertexColorShaderDraw {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub trait VertexColorShaderDrawLines {
    fn draw_lines<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}
