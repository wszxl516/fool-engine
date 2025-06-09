use crate::map2anyhow_error;
pub use crate::resource::ResourceManager;
use crate::scheduler::FrameScheduler;
use crate::script::graphics::window::LuaWindow;
use crate::script::{run_init_fn, setup_modules, EguiContext};
use fool_graphics::GraphRender;
use fool_script::{thread::AsyncScheduler, FoolScript};
use fool_window::EventProxy;
use fool_window::WinEvent;
use std::sync::Arc;
use winit::window::Window;
pub mod event;
pub mod script;
pub struct Engine {
    resource: ResourceManager,
    script: FoolScript,
    window: Option<Arc<Window>>,
    render: Option<GraphRender>,
    proxy: Option<EventProxy>,
    scheduler: FrameScheduler,
    script_scheduler: AsyncScheduler,
    lua_window: Option<LuaWindow>,
    lua_gui: Option<EguiContext>,
    events_current_frame: Vec<WinEvent>,
}

impl Engine {
    pub fn new(fps: u32) -> anyhow::Result<Self> {
        let resource = ResourceManager::new()?;
        let mut script = FoolScript::new(resource.raw_resource.clone())?;
        script.setup()?;
        setup_modules(&script)?;
        map2anyhow_error!(script.load_main(), "load main.lua failed: ")?;
        Ok(Engine {
            resource,
            script: script.clone(),
            window: None,
            proxy: None,
            render: None,
            scheduler: FrameScheduler::new(fps),
            script_scheduler: AsyncScheduler::new(&script, 1),
            lua_window: None,
            lua_gui: None,
            events_current_frame: Vec::new(),
        })
    }

    pub fn init(&mut self, window: Arc<Window>, proxy: &EventProxy) -> anyhow::Result<()> {
        self.window.replace(window.clone());
        let render = GraphRender::new(window.clone())?;
        self.resource
            .setup_egui_texture_fallback(render.gui_context());
        egui_extras::install_image_loaders(render.gui_context());
        let size = window.inner_size();
        let mut lua_window = LuaWindow {
            window: window.clone(),
            resource: self.resource.clone(),
            proxy: proxy.clone(),
            on_exit: None,
        };
        let mut lua_gui = EguiContext {
            context: render.gui_context().clone(),
            width: size.width as _,
            heigth: size.height as _,
            resource: self.resource.clone(),
        };
        self.proxy.replace(proxy.clone());
        run_init_fn(&self.script, &mut lua_gui, &mut lua_window)?;
        self.lua_gui.replace(lua_gui);
        self.lua_window.replace(lua_window);
        self.render.replace(render);
        self.resource
            .scene_graph
            .write()
            .center_with_screen_size(size.width as f64, size.height as f64);
        Ok(())
    }
    pub fn stop(&mut self) {
        log::info!("stop engine");
        self.scheduler.pause();
        if let Some(proxy) = &self.proxy {
            let _ = proxy.exit();
        }
    }
    fn lua_exit_callback(&self) -> bool {
        self.lua_window
            .as_ref()
            .and_then(|w| w.on_exit.as_ref())
            .and_then(|f| f.call::<bool>(()).ok())
            .unwrap_or(false)
    }
    fn exiting(&mut self) {
        self.lua_exit_callback();
        if let (Some(render), Some(window)) = (self.render.take(), self.window.take()) {
            drop(window);
            drop(render);
        }
        log::debug!("exiting engine");
    }
}
