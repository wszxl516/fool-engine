pub mod event;
pub mod lua;
pub mod physics;
pub mod resource;
mod scheduler;
pub mod utils;
use event::{EngineEvent, EngineEventLoop, EventState};
use fool_graphics::GraphRender;
use lua::gui::Gui;
use lua::LuaBindings;
use parking_lot::Mutex;
use resource::ResourceManager;
use scheduler::Scheduler;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoopBuilder, EventLoopProxy},
    platform::x11::{EventLoopBuilderExtX11, WindowAttributesExtX11},
    window::{Window, WindowAttributes},
};
struct Engine {
    resource: Arc<Mutex<ResourceManager>>,
    lua: LuaBindings,
    event_state: EventState,
    pub window_attr: WindowAttributes,
    pub window: Option<Arc<Window>>,
    pub render: Option<GraphRender>,
    engine_event_loop: EngineEventLoop,
    scheduler: Scheduler,
}

//init
impl Engine {
    pub fn new(
        fps: u32,
        window_attr: WindowAttributes,
        event_proxy: EventLoopProxy<EngineEvent>,
    ) -> anyhow::Result<Self> {
        let mut lua = map2anyhow_error!(LuaBindings::new(), "init LuaBindings failed")?;
        let gui = Gui::new(&lua);
        let resource = Arc::new(Mutex::new(ResourceManager::new()?));
        map2anyhow_error!(gui.init(), "gui init failed")?;
        let event_proxy = EngineEventLoop::new(event_proxy);
        lua.setup(resource.clone(), event_proxy.clone())?;
        map2anyhow_error!(lua.load_main(), "load main.lua failed: ")?;
        Ok(Engine {
            resource,
            lua,
            event_state: EventState::new(event_proxy.clone()),
            window: None,
            window_attr,
            render: None,
            scheduler: Scheduler::new(fps),
            engine_event_loop: event_proxy,
        })
    }

    pub fn init(
        &mut self,
        window: Arc<Window>,
        _event_loop: &ActiveEventLoop,
    ) -> anyhow::Result<()> {
        self.window.replace(window.clone());
        let render = GraphRender::new(window.clone())?;
        if let Err(err) = self.lua.run_init_fn(
            render.gui_context(),
            window.clone(),
            self.resource.clone(),
            self.engine_event_loop.clone(),
        ) {
            log_error_exit!("run lua init failed: {}", err)
        }
        egui_extras::install_image_loaders(render.gui_context());
        self.render.replace(render);
        Ok(())
    }
}

// graphics
impl Engine {
    pub fn graphics(&mut self) {
        if let Some(render) = &mut self.render {
            render.clear_view();
            render.graphics(fool_graphics::test::test_graph);
        }
    }
    pub fn gui(&mut self) {
        let resource = self.resource.clone();
        if let (Some(render), Some(window)) = (&mut self.render, &self.window) {
            render.gui(|ctx| {
                let lua = self.lua.clone();
                if let Err(err) = lua.run_view_fn(
                    ctx.clone(),
                    resource.clone(),
                    window.clone(),
                    self.engine_event_loop.clone(),
                ) {
                    log_error_exit!("run lua view failed: {}", err)
                }
            });
        }
    }
}
//event
impl Engine {
    fn window_event(&mut self, event: &winit::event::WindowEvent) {
        self.event_state.handle_event(event);
        if let Some(render) = &mut self.render {
            render.gui_event(&event);
        }
        if let Some(window) = &self.window {
            let resource = self.resource.clone();
            if let Err(err) = self.lua.run_event_fn(
                &mut self.event_state,
                window.clone(),
                resource,
                self.engine_event_loop.clone(),
            ) {
                log_error_exit!("run lua event failed: {}", err)
            }
        }
        match event {
            WindowEvent::Resized(size) => {
                if let (Some(render), Some(window)) = (&mut self.render, &self.window) {
                    render.resize(size.width, size.height);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                self.graphics();
                self.gui()
            }
            _ => {}
        }
    }
    fn engine_event(&mut self, event_loop: &ActiveEventLoop, event: EngineEvent) {
        match event {
            EngineEvent::LoadCursor(cursor) => {
                if let Err(err) = self.resource.lock().load_cursor(&cursor, event_loop) {
                    log::error!("load_cursor {} failed: {}", cursor, err)
                }
            }
            EngineEvent::ExitWindow => {
                log::debug!("exit window");
                event_loop.exit()
            }
            EngineEvent::LoadUITexture(path) => {
                if let Some(render) = &self.render {
                    if let Err(err) = self
                        .resource
                        .lock()
                        .load_ui_texture(&path, render.gui_context())
                    {
                        log::error!("load uitexture {} failed: {}", path, err)
                    }
                }
            }
            _ => {}
        }
    }
}
impl ApplicationHandler<EngineEvent> for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(self.window_attr.clone())
                .unwrap_or_else(|err| log_error_exit!("create_window failed: {}", err));
            let window = Arc::new(window);
            self.init(window.clone(), event_loop)
                .unwrap_or_else(|err| log_error_exit!("init engine failed: {}", err));
        }
        event_loop.set_control_flow(ControlFlow::Wait);
    }
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.window_event(&event);
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.scheduler.trigger_redraw(event_loop) {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: EngineEvent) {
        self.engine_event(event_loop, event);
    }
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(render), Some(window)) = (self.render.take(), self.window.take()) {
            drop(window);
            drop(render);
        }
        log::debug!("exiting window");
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
    let event_loop = EventLoopBuilder::<EngineEvent>::default()
        .with_x11()
        .with_any_thread(true)
        .build()?;
    let event_proxy = event_loop.create_proxy();
    let mut engine = Engine::new(30, window_attr, event_proxy)?;
    event_loop
        .run_app(&mut engine)
        .expect("Couldn't run event loop");
    Ok(())
}
