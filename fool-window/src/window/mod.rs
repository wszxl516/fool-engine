mod app;
pub mod event;
pub mod input;
pub mod proxy;
pub use app::{Application, CustomEvent};
pub use event::{AppEvent, WindowCursor};
pub use input::WinEvent;
pub use proxy::EventProxy;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    window::{Cursor, Window, WindowAttributes},
};
use winit::{event_loop::OwnedDisplayHandle, monitor::MonitorHandle, window::Theme};
pub struct FoolWindow {
    window: Option<Arc<Window>>,
    window_attr: WindowAttributes,
    proxy: EventProxy,
    app: Box<dyn Application + 'static>,
    event_loop: Option<EventLoop<AppEvent>>,
    cursor: Option<Cursor>,
    available_monitors: Option<Arc<Vec<MonitorHandle>>>,
    primary_monitor: Option<Arc<MonitorHandle>>,
    owned_display_handle: Option<Arc<OwnedDisplayHandle>>,
    system_theme: Option<Arc<Theme>>,
    input: WinEvent,
}
impl FoolWindow {
    pub fn new(
        window_attr: WindowAttributes,
        app: impl Application + 'static,
        event_loop: EventLoop<AppEvent>,
    ) -> anyhow::Result<Self> {
        let event_proxy = event_loop.create_proxy();
        Ok(FoolWindow {
            window: None,
            window_attr,
            app: Box::new(app),
            proxy: EventProxy {
                proxy: Arc::new(event_proxy),
            },
            event_loop: Some(event_loop),
            cursor: None,
            available_monitors: None,
            primary_monitor: None,
            owned_display_handle: None,
            system_theme: None,
            input: WinEvent::new(),
        })
    }
    pub fn init(&mut self, window: Arc<Window>, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        self.available_monitors
            .replace(Arc::new(event_loop.available_monitors().collect()));
        if let Some(monitor) = event_loop.primary_monitor() {
            self.primary_monitor.replace(Arc::new(monitor));
        }
        self.owned_display_handle
            .replace(Arc::new(event_loop.owned_display_handle()));
        if let Some(theme) = event_loop.system_theme() {
            self.system_theme.replace(Arc::new(theme));
        }
        self.window.replace(window.clone());
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        if let Some(event_loop) = self.event_loop.take() {
            event_loop.run_app(self)?
        }
        Ok(())
    }
    pub fn set_cursor(&mut self, icon: Cursor) {
        self.cursor = Some(icon);
    }
}

impl ApplicationHandler<AppEvent> for FoolWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            match event_loop.create_window(self.window_attr.clone()) {
                Ok(window) => {
                    let window = Arc::new(window);
                    self.init(window.clone(), event_loop);
                    self.app.init(window, &self.proxy);
                }
                Err(err) => {
                    log::error!("create window failed: {}", err);
                    event_loop.exit();
                }
            }
        }
        event_loop.set_control_flow(ControlFlow::Wait);
    }
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.input.step_with_window_events(&[&event]);
        self.app.event(&self.input, &event);
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(window), Some(cursor)) = (&self.window, &self.cursor) {
            window.set_cursor(cursor.clone());
        }
        self.app.update();
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::SetCursor(cursor) => match cursor.to_cursor(event_loop) {
                Ok(cursor) => {
                    self.cursor = cursor;
                    log::trace!("set cursor succeed!")
                }
                Err(err) => log::error!("set cursor failed: {}", err),
            },
            AppEvent::Exit => event_loop.exit(),
            AppEvent::CustomEvent(ev) => {
                self.app.user_event(ev);
            }
            _ => {}
        }
    }
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.app.exiting();
    }
}
