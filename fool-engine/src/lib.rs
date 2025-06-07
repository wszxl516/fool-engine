pub mod config;
pub mod engine;
pub mod event;
pub mod physics;
pub mod resource;
mod scheduler;
pub mod script;
pub mod utils;
use event::EngineEvent;
use winit::{event_loop::EventLoopBuilder, platform::x11::EventLoopBuilderExtX11};
pub fn init_engine() -> anyhow::Result<()> {
    // let window_attr = Window::default_attributes()
    //     .with_base_size(Size::Logical(LogicalSize {
    //         width: 800.0,
    //         height: 600.0,
    //     }))
    //     .with_resizable(true)
    //     .with_title("Test Engine");
    let config = config::Config::from_file()?;
    let event_loop = EventLoopBuilder::<EngineEvent>::default()
        .with_x11()
        .with_any_thread(true)
        .build()?;
    let window_attr = config.window.build(&event_loop)?;
    let event_proxy = event_loop.create_proxy();
    let mut engine = engine::Engine::new(30, window_attr, event_proxy)?;
    event_loop
        .run_app(&mut engine)
        .expect("Couldn't run event loop");
    Ok(())
}
