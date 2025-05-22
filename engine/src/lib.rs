pub mod graphics;
mod gui;
pub mod input;
pub mod lua;
pub mod physics;
pub mod resource;
pub mod utils;
use gui::Gui;
use input::InputState;
use log::error;
use lua::LuaBindings;
use nannou::{prelude::*, wgpu::Backends};
use resource::ResourceManager;
use std::sync::{Arc, Mutex};

struct Engine {
    resource: Arc<Mutex<ResourceManager>>,
    lua: LuaBindings,
    input: InputState,
    gui: Gui,
}
impl Engine {
    pub fn new(app: &App) -> anyhow::Result<Self> {
        app.set_exit_on_escape(false);
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
        map2anyhow_error!(lua.run_init_fn(), "run_init_fn failed: ")?;
        Ok(Engine {
            resource,
            lua,
            input: InputState::default(),
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
                .unwrap_or_else(|e| error!("run_draw_fn from lua failed: {}", e));
        }

        draw.to_frame(app, &frame)
            .unwrap_or_else(|e| error!("draw to_frame failed: {:?}", e));
        model.gui.end(&frame).unwrap();
    }
    fn update(_app: &App, model: &mut Engine, _update: Update) {
        model
            .lua
            .run_update_fn()
            .unwrap_or_else(|e| error!("run lua run_update_fn failed: {:?}", e));
    }
    fn event(_app: &App, model: &mut Engine, _event: Event) {
        model.input.begin_frame();
        match _event {
            Event::WindowEvent { id: _, simple } => {
                if let Some(e) = simple {
                    model.input.handle_event(&e);
                }
            }
            _ => {}
        }
        model
            .lua
            .run_event_fn(&model.input)
            .unwrap_or_else(|e| error!("run lua run_event_fn failed: {:?}", e));
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
    .run();
    Ok(())
}
