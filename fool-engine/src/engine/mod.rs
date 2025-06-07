use crate::event::EngineEvent;
use crate::event::{EngineEventLoop, EventState};
use crate::script::{run_init_fn, setup_modules};
use crate::map2anyhow_error;
pub use crate::resource::ResourceManager;
use crate::scheduler::Scheduler;
use fool_graphics::GraphRender;
use fool_script::{thread::AsyncScheduler, FoolScript};
use std::sync::Arc;
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes};
pub mod event;
pub mod script;

pub struct Engine {
    resource: ResourceManager,
    script: FoolScript,
    event_state: EventState,
    window_attr: WindowAttributes,
    window: Option<Arc<Window>>,
    render: Option<GraphRender>,
    engine_event_loop: EngineEventLoop,
    scheduler: Scheduler,
    script_scheduler: AsyncScheduler,
}

impl Engine {
    pub fn new(
        fps: u32,
        window_attr: WindowAttributes,
        event_proxy: EventLoopProxy<EngineEvent>,
    ) -> anyhow::Result<Self> {
        let resource = ResourceManager::new()?;
        let mut script = FoolScript::new(resource.raw_resource.clone())?;
        let event_proxy = EngineEventLoop::new(event_proxy);
        script.setup()?;
        setup_modules(&script, resource.clone(), event_proxy.clone())?;
        map2anyhow_error!(script.load_main(), "load main.lua failed: ")?;
        Ok(Engine {
            resource,
            script: script.clone(),
            event_state: EventState::new(event_proxy.clone()),
            window: None,
            window_attr,
            render: None,
            scheduler: Scheduler::new(fps),
            engine_event_loop: event_proxy,
            script_scheduler: AsyncScheduler::new(&script, 1),
        })
    }

    pub fn init(
        &mut self,
        window: Arc<Window>,
        _event_loop: &ActiveEventLoop,
    ) -> anyhow::Result<()> {
        self.window.replace(window.clone());
        let render = GraphRender::new(window.clone())?;
        self.resource
            .setup_egui_texture_fallback(render.gui_context());
        egui_extras::install_image_loaders(render.gui_context());
        run_init_fn(
            &self.script,
            render.gui_context(),
            window.clone(),
            self.resource.clone(),
            self.engine_event_loop.clone(),
        )?;
        self.render.replace(render);
        let size = window.inner_size();
        self.resource
            .scene_graph
            .write()
            .center_with_screen_size(size.width as f64, size.height as f64);
        Ok(())
    }
    pub fn stop(&mut self) {
        log::info!("stop engine");
        self.scheduler.pause();
        self.engine_event_loop.exit_window();
    }
}
