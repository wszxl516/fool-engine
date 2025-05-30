pub mod event;
pub mod graphics;
mod gui;
pub mod lua;
pub mod physics;
pub mod resource;
pub mod utils;
use event::EventState;
use fool_graphics::{App, AppModel};
use gui::Gui;
use lua::LuaBindings;
use parking_lot::Mutex;
use resource::ResourceManager;
use std::sync::Arc;
use winit::{
    dpi::{LogicalSize, Size},
    event_loop::ActiveEventLoop,
    event_loop::EventLoop,
    platform::x11::WindowAttributesExtX11,
    window::Window,
};
struct Engine {
    resource: Arc<Mutex<ResourceManager>>,
    lua: LuaBindings,
    event_state: EventState,
}
impl Engine {
    pub fn new() -> anyhow::Result<Self> {
        let mut lua = map2anyhow_error!(LuaBindings::new(), "init LuaBindings failed")?;
        let gui = Gui::new(&lua);
        let resource = Arc::new(Mutex::new(ResourceManager::new()?));
        map2anyhow_error!(gui.init(), "gui init failed")?;
        lua.setup(resource.clone())?;
        map2anyhow_error!(lua.load_main(), "load main.lua failed: ")?;
        Ok(Engine {
            resource,
            lua,
            event_state: EventState::default(),
        })
    }
}
impl AppModel for Engine {
    fn init(&mut self, context: &egui::Context, window: Arc<winit::window::Window>) {
        if let Err(err) = self.lua.run_init_fn(context, window, self.resource.clone()) {
            log_error_exit!("run lua init failed: {}", err)
        }
    }
    fn gui(&mut self, context: &egui::Context, window: Arc<winit::window::Window>) {
        let resource = self.resource.clone();
        let lua = self.lua.clone();
        if let Err(err) = lua.run_view_fn(context.clone(), resource.clone(), window.clone()) {
            log_error_exit!("run lua view failed: {}", err)
        }
    }
    fn window_event(&mut self, event: &winit::event::WindowEvent) {
        self.event_state.begin_frame();
        self.event_state.handle_event(event);
    }
    fn event_loop(&mut self, event_loop: &ActiveEventLoop) {
        let _ = self.resource.lock().load_cursor(event_loop);
    }
}
pub fn init_engine() -> anyhow::Result<()> {
    let window_attr = Window::default_attributes()
        .with_base_size(Size::Logical(LogicalSize {
            width: 800.0,
            height: 600.0,
        }))
        .with_resizable(true)
        .with_title("Test Engine");
    let engine = Engine::new()?;
    let mut app = App::new(30, window_attr, Box::new(engine));
    let event_loop = EventLoop::new()?;
    event_loop
        .run_app(&mut app)
        .expect("Couldn't run event loop");
    Ok(())
}
#[macro_export]
macro_rules! log_error_exit {
    ($($arg:tt)+) => ({
        log::error!($($arg)+);
        std::process::exit(-1);
    });
}
