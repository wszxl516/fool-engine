use crate::config::BaseConfig;
use crate::map2anyhow_error;
pub use crate::resource::ResourceManager;
use crate::scheduler::FrameScheduler;
use crate::script::LuaEngine;
use crate::script::{run_init_fn, setup_modules};
use fool_graphics::canvas::SceneGraph;
use fool_graphics::GraphRender;
use fool_script::{thread::AsyncScheduler, FoolScript};
use fool_window::EventProxy;
use fool_window::WinEvent;
use parking_lot::RwLock;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use winit::window::Window;
pub mod event;
pub mod script;
mod status;
pub use status::EngineStatus;
pub struct Engine {
    resource: ResourceManager,
    script: FoolScript,
    window: Option<Arc<Window>>,
    render: Option<GraphRender>,
    event_proxy: Option<EventProxy>,
    scheduler: FrameScheduler,
    script_scheduler: AsyncScheduler,
    lua_engine: Option<LuaEngine>,
    scene_graph: Arc<RwLock<SceneGraph>>,
    events_current_frame: Vec<WinEvent>,
    frame_capture: VecDeque<PathBuf>,
    status: Arc<RwLock<EngineStatus>>,
    base_config: BaseConfig,
}

impl Engine {
    pub fn new(base_config: BaseConfig) -> anyhow::Result<Self> {
        let base_config = base_config.build()?;
        log::debug!("engine base config: {:?}", base_config);
        let resource = ResourceManager::new(base_config.assets_path.clone())?;
        let mut script = FoolScript::new(resource.raw_resource.clone())?;
        script.setup()?;
        setup_modules(&script)?;
        let scene_graph = Arc::new(RwLock::new(SceneGraph {
            font_mgr: resource.graphics_font.clone(),
            img_mgr: resource.graphics_img.clone(),
            ..Default::default()
        }));
        map2anyhow_error!(script.load_main(), "load main.lua failed: ")?;
        Ok(Engine {
            resource,
            script: script.clone(),
            window: None,
            event_proxy: None,
            render: None,
            scheduler: FrameScheduler::new(base_config.fps),
            script_scheduler: AsyncScheduler::new(script.modules.clone()),
            lua_engine: None,
            events_current_frame: Vec::new(),
            frame_capture: Default::default(),
            status: Arc::new(RwLock::new(EngineStatus::Init)),
            base_config,
            scene_graph,
        })
    }

    pub fn init(&mut self, window: Arc<Window>, proxy: &EventProxy) -> anyhow::Result<()> {
        self.window.replace(window.clone());
        let render = GraphRender::new(window.clone())?;
        self.resource
            .setup_egui_texture_fallback(render.gui_context());
        egui_extras::install_image_loaders(render.gui_context());
        let size = window.inner_size();
        let lua_engine = LuaEngine::new(
            window,
            render.gui_context().clone(),
            proxy.clone(),
            self.resource.clone(),
            self.scene_graph.clone(),
            self.status.clone(),
        )?;
        self.event_proxy.replace(proxy.clone());
        run_init_fn(&self.script, &lua_engine)?;
        self.lua_engine.replace(lua_engine);
        self.render.replace(render);
        self.script_scheduler.init()?;
        self.scene_graph
            .write()
            .center_with_screen_size(size.width as f64, size.height as f64);
        *self.status.write() = EngineStatus::Running;
        Ok(())
    }
    pub fn stop(&mut self) {
        log::info!("stop engine");
        self.scheduler.pause();
        *self.status.write() = EngineStatus::Exiting;
        if let Some(proxy) = &self.event_proxy {
            let _ = proxy.exit();
        }
    }
    fn exiting(&mut self) {
        self.script_scheduler.stop_all();
        if let (Some(render), Some(window), Some(lua_engine)) = (
            self.render.take(),
            self.window.take(),
            self.lua_engine.take(),
        ) {
            drop(window);
            drop(render);
            drop(lua_engine);
        }
        log::debug!("exiting engine");
    }
}
