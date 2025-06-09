pub mod config;
pub mod engine;
pub mod event;
pub mod physics;
pub mod resource;
mod scheduler;
pub mod script;
pub mod utils;
use fool_window::{AppEvent, FoolWindow};
use winit::{event_loop::EventLoopBuilder, platform::x11::EventLoopBuilderExtX11};

pub fn init_engine() -> anyhow::Result<()> {
    let config = config::Config::from_file()?;
    let event_loop = EventLoopBuilder::<AppEvent>::default()
        .with_x11()
        .with_any_thread(true)
        .build()?;
    let window_attr = config.window.build(&event_loop)?;
    let engine = engine::Engine::new(config.base)?;
    let mut window = FoolWindow::new(window_attr, engine, event_loop)?;
    window.run()?;
    Ok(())
}
