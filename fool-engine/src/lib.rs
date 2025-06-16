pub mod config;
pub mod engine;
pub mod event;
pub mod physics;
pub mod resource;
pub mod save;
mod scheduler;
pub mod script;
pub mod utils;
use fool_window::{AppEvent, FoolWindow};
use winit::event_loop::EventLoopBuilder;

pub fn init_engine() -> anyhow::Result<()> {
    let config = config::Config::from_file()?;
    let event_loop = EventLoopBuilder::<AppEvent>::default().build()?;
    let window_attr = config.window.build(&event_loop)?;
    let engine = engine::Engine::new(config.base)?;
    let mut window = FoolWindow::new(window_attr, engine, event_loop)?;
    window.run()?;
    Ok(())
}
