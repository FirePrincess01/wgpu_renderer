//! Holds an instance to a user app which can be created at a later point in time
//! 
//! Details: This object is meant to be used dynamically so template cascading can be avoided

use crate::{default_application::default_application_interface::{DefaultApplicationInterface, DefaultApplicationInterfaceCreate, DefaultApplicationInterfaceRuntime}, wgpu_renderer::WgpuRendererInterface};


pub struct UserAppBuilder<UserApp: DefaultApplicationInterface> {
    user_app: Option<UserApp>,
}

impl<UserApp: DefaultApplicationInterface> UserAppBuilder<UserApp> {
    pub fn new() -> Self {
        Self { user_app: None }
    }
}

impl <UserApp: DefaultApplicationInterface> UserAppBuilderInterface for UserAppBuilder<UserApp> {

    fn create(&mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    ) {
        self.user_app = Some(UserApp::create(renderer_interface, size, scale_factor))
    }

    fn get<'a>(&'a mut self) -> Option<&'a mut dyn DefaultApplicationInterfaceRuntime> {
        match &mut self.user_app {
            Some(user_app) => Some(user_app),
            None => None,
        }
    }
}

pub trait UserAppBuilderInterface {
    fn create(&mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        size: winit::dpi::PhysicalSize<u32>,
        scale_factor: f32,
    );

    fn get<'a>(&'a mut self) -> Option<&'a mut dyn DefaultApplicationInterfaceRuntime>;
}