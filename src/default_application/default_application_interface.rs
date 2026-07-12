//! Interface for the user to create applications

use crate::wgpu_renderer::WgpuRendererInterface;


pub trait DefaultApplicationInterfaceCreate {
    fn create(
        renderer_interface: &mut dyn WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    ) -> Self;
}

pub trait DefaultApplicationInterfaceRuntime {
    fn get_size(&self) -> winit::dpi::PhysicalSize<u32>;
    fn resize(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        new_size: winit::dpi::PhysicalSize<u32>,
    );
    fn update_scale_factor(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        scale_factor: f32,
    );
    fn update(&mut self, renderer_interface: &mut dyn WgpuRendererInterface, dt: instant::Duration);
    fn input(&mut self, event: &winit::event::WindowEvent) -> bool;
    fn render(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
    ) -> Result<(), ()>;
}

pub trait DefaultApplicationInterface:
    DefaultApplicationInterfaceCreate + DefaultApplicationInterfaceRuntime {}

impl<T> DefaultApplicationInterface for T
where
    T: DefaultApplicationInterfaceCreate + DefaultApplicationInterfaceRuntime,
{}