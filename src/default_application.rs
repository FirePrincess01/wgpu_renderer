//! Using an event loop to call functions of a basic wgpu renderer application

mod state;
pub mod default_application_interface;
mod user_app_builder;
mod app;

use std::sync::Arc;

use winit::{dpi::LogicalSize, event_loop::{self, EventLoop}, window};

use crate::{default_application::{app::{App, UserEvent}, default_application_interface::DefaultApplicationInterface, state::State, user_app_builder::{UserAppBuilder, UserAppBuilderInterface}}, wgpu_renderer::WgpuRendererInterface};

pub fn init_env_logger() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Info).unwrap();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // env_logger::init();
        let mut builder = env_logger::Builder::new();
        builder.target(env_logger::Target::Stdout);
        builder.filter_level(log::LevelFilter::Info);
        builder.write_style(env_logger::WriteStyle::Always);

        builder.init();
    }
    log::info!("Logger initialized");
}

pub fn create_event_loop() -> winit::event_loop::EventLoop<UserEvent> {
    let event_loop = winit::event_loop::EventLoop::<UserEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    event_loop
}



pub fn run_app<UserApp: DefaultApplicationInterface + 'static>(event_loop: EventLoop<UserEvent>) {
    let app = App {
        proxy: Some(event_loop.create_proxy()),
        state: None,
        user_app: Box::new(UserAppBuilder::<UserApp>::new()),
        last_render_time: instant::Instant::now(),
        initial_size: None,
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app = app;
        event_loop.run_app(&mut app).unwrap();
    }
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::EventLoopExtWebSys;
        event_loop.spawn_app(app);
    }
}



