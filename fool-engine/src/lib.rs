pub mod engine;
pub mod event;
pub mod script;
pub mod physics;
pub mod resource;
mod scheduler;
pub mod utils;
use event::EngineEvent;
use winit::{
    dpi::{LogicalSize, Size},
    event_loop::EventLoopBuilder,
    platform::x11::{EventLoopBuilderExtX11, WindowAttributesExtX11},
    window::Window,
};
pub fn init_engine() -> anyhow::Result<()> {
    let window_attr = Window::default_attributes()
        .with_base_size(Size::Logical(LogicalSize {
            width: 800.0,
            height: 600.0,
        }))
        .with_resizable(true)
        .with_title("Test Engine");
    let event_loop = EventLoopBuilder::<EngineEvent>::default()
        .with_x11()
        .with_any_thread(true)
        .build()?;
    let event_proxy = event_loop.create_proxy();
    let mut engine = engine::Engine::new(30, window_attr, event_proxy)?;
    event_loop
        .run_app(&mut engine)
        .expect("Couldn't run event loop");
    Ok(())
}
