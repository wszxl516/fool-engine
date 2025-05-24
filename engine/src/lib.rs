pub mod event;
pub mod graphics;
mod gui;
pub mod lua;
pub mod physics;
pub mod resource;
pub mod utils;
use event::EventState;
use gui::Gui;
use log::error;
use lua::LuaBindings;
use nannou::{prelude::*, wgpu::Backends};
use parking_lot::Mutex;
use resource::ResourceManager;
use std::sync::Arc;
struct Engine {
    resource: Arc<Mutex<ResourceManager>>,
    lua: LuaBindings,
    event_state: EventState,
    gui: Gui,
}
impl Engine {
    pub fn new(app: &App) -> anyhow::Result<Self> {
        app.set_exit_on_escape(false);
        // app.set_loop_mode(mode);
        let mut lua = map2anyhow_error!(LuaBindings::new(), "init LuaBindings failed")?;
        let window_id = Self::init_window(app)?;
        let window = app.window(window_id).unwrap();
        let gui = Gui::new(&window, &lua);
        let ctx = gui.egui.clone();
        let ctx = ctx.borrow().ctx().clone();
        let dev = window.device_queue_pair();
        let resource = Arc::new(Mutex::new(ResourceManager::new(dev, ctx.clone())?));
        map2anyhow_error!(gui.init(), "gui init failed")?;
        lua.setup(resource.clone())?;
        map2anyhow_error!(lua.load_main(), "load main.lua failed: ")?;
        lua.run_init_fn()
            .unwrap_or_else(|err| log_error_exit!("{}", err));
        Ok(Engine {
            resource,
            lua,
            event_state: EventState::default(),
            gui,
        })
    }
    pub fn init_window(app: &App) -> anyhow::Result<WindowId> {
        map2anyhow_error!(
            app.new_window()
                .size(800, 800)
                .msaa_samples(4)
                .decorations(true)
                .transparent(true)
                .raw_event(|_app, model: &mut Engine, event| {
                    model.gui.event(event);
                    model.event_state.handle_event(event);
                })
                .build(),
            "init window failed: {}"
        )
    }
    fn view(app: &App, model: &Engine, frame: Frame) {
        let draw = app.draw();
        draw.background().color(BLACK);
        {
            let mut ctx = model.gui.egui.borrow_mut();
            let ctx = ctx.begin_frame();
            model
                .lua
                .run_draw_fn(draw.clone(), ctx.context(), app, model.resource.clone())
                .unwrap_or_else(|e| log_error_exit!("run_draw_fn from lua failed: {}", e));
        }

        draw.to_frame(app, &frame)
            .unwrap_or_else(|e| error!("draw to_frame failed: {:?}", e));
        model.gui.end(&frame).unwrap();
    }
    fn update(_app: &App, model: &mut Engine, _update: Update) {
        model.event_state.begin_frame();
        model
            .lua
            .run_update_fn()
            .unwrap_or_else(|e| log_error_exit!("run lua run_update_fn failed: {:?}", e));
    }
    fn event(_app: &App, model: &mut Engine, _event: Event) {
        model
            .lua
            .run_event_fn(&mut model.event_state)
            .unwrap_or_else(|e| log_error_exit!("run lua run_event_fn failed: {:?}", e));
        *_app.is_exit.borrow_mut() = model.event_state.is_exit;
    }
    fn exit(_app: &App, _model: Engine) {
        _app.quit();
        std::process::exit(0);
    }
}

pub fn init_engine() -> anyhow::Result<()> {
    nannou::app::Builder::new(|app| {
        Engine::new(app).unwrap_or_else(|e| {
            error!("init Model {:?}", e);
            std::process::exit(1)
        })
    })
    .update(Engine::update)
    .view(Engine::view)
    .loop_mode(LoopMode::RefreshSync)
    .backends(Backends::all())
    .size(800, 800)
    .event(Engine::event)
    .exit(Engine::exit)
    .run();
    Ok(())
}
#[macro_export]
macro_rules! log_error_exit {
    ($($arg:tt)+) => ({
        log::error!($($arg)+);
        std::process::exit(-1);
    });
}
