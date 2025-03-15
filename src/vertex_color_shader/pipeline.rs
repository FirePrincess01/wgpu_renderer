//! A general purpose pipeline using vertices, colors and instances
//!
//! Vertices and Colors are independently updateable
//! The implementation uses wgpu for rendering
//!

use super::super::wgpu_renderer::depth_texture;
use super::camera_bind_group_layout;
use super::color;
use super::instance;
use super::vertex;
use super::vertex_color_shader_draw::VertexColorShaderDrawLines;
use super::CameraUniformBuffer;
use super::VertexColorShaderDraw;

/// A general purpose shader using vertices, colors and an instance matrix
#[allow(dead_code)]
pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new_lines(
        device: &wgpu::Device,
        camera_bind_group_layout: &camera_bind_group_layout::CameraBindGroupLayout,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        Self::new_parameterized(
            device,
            camera_bind_group_layout,
            surface_format,
            wgpu::PrimitiveTopology::LineList,
        )
    }

    pub fn new(
        device: &wgpu::Device,
        camera_bind_group_layout: &camera_bind_group_layout::CameraBindGroupLayout,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        Self::new_parameterized(
            device,
            camera_bind_group_layout,
            surface_format,
            wgpu::PrimitiveTopology::TriangleList,
        )
    }

    fn new_parameterized(
        device: &wgpu::Device,
        camera_bind_group_layout: &camera_bind_group_layout::CameraBindGroupLayout,
        surface_format: wgpu::TextureFormat,
        topology: wgpu::PrimitiveTopology,
    ) -> Self {
        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[camera_bind_group_layout.get()],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    vertex::Vertex::desc(),
                    color::Color::desc(),
                    instance::InstanceRaw::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology, // wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // counter-clockwise direction
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_texture::DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self { render_pipeline }
    }

    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
    }

    pub fn draw<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera: &'a CameraUniformBuffer,
        mesh: &'a dyn VertexColorShaderDraw,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        camera.bind(render_pass);
        mesh.draw(render_pass);
    }

    pub fn draw_lines<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera: &'a CameraUniformBuffer,
        mesh: &'a dyn VertexColorShaderDrawLines,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);
        camera.bind(render_pass);
        mesh.draw_lines(render_pass);
    }
}
