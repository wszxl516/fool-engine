use crate::map2anyhow_error;
pub use crate::resource::ResourceManager;
use crate::scheduler::Scheduler;
use crate::script::{run_init_fn, setup_modules};
use fool_graphics::GraphRender;
use fool_script::{thread::AsyncScheduler, FoolScript};
use fool_window::EventProxy;
use std::sync::Arc;
use winit::window::Window;
pub mod event;
pub mod script;
pub struct Engine {
    resource: ResourceManager,
    script: FoolScript,
    // event_state: EventState,
    window: Option<Arc<Window>>,
    render: Option<GraphRender>,
    proxy: Option<EventProxy>,
    scheduler: Scheduler,
    script_scheduler: AsyncScheduler,
}

impl Engine {
    pub fn new(fps: u32) -> anyhow::Result<Self> {
        let resource = ResourceManager::new()?;
        let mut script = FoolScript::new(resource.raw_resource.clone())?;
        script.setup()?;
        setup_modules(&script, resource.clone())?;
        map2anyhow_error!(script.load_main(), "load main.lua failed: ")?;
        Ok(Engine {
            resource,
            script: script.clone(),
            window: None,
            proxy: None,
            render: None,
            scheduler: Scheduler::new(fps),
            script_scheduler: AsyncScheduler::new(&script, 1),
        })
    }

    pub fn init(&mut self, window: Arc<Window>, proxy: &EventProxy) -> anyhow::Result<()> {
        self.window.replace(window.clone());
        let render = GraphRender::new(window.clone())?;
        self.resource
            .setup_egui_texture_fallback(render.gui_context());
        egui_extras::install_image_loaders(render.gui_context());
        self.proxy.replace(proxy.clone());
        run_init_fn(
            &self.script,
            render.gui_context(),
            window.clone(),
            self.resource.clone(),
            proxy.clone(),
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
        if let Some(proxy) = &self.proxy {
            let _ = proxy.exit();
        }
    }
    fn exiting(&mut self) {
        if let (Some(render), Some(window)) = (self.render.take(), self.window.take()) {
            drop(window);
            drop(render);
        }
        log::debug!("exiting window");
    }
}
