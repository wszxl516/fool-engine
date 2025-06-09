use super::{AppEvent, Arc, EventLoopProxy};
use crate::window::event::WindowCursor;
use winit::event_loop::ControlFlow;
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EventProxy {
    pub proxy: Arc<EventLoopProxy<AppEvent>>,
}

impl EventProxy {
    pub fn send(&self, event: AppEvent) -> anyhow::Result<()> {
        Ok(self.proxy.send_event(event)?)
    }
    pub fn exit(&self) -> anyhow::Result<()> {
        self.send(AppEvent::Exit)
    }
    pub fn set_cursor(&self, cursor: WindowCursor) -> anyhow::Result<()> {
        self.send(AppEvent::SetCursor(cursor))
    }
    pub fn wait(&self) -> anyhow::Result<()> {
        self.send(AppEvent::ControlFlow(ControlFlow::Wait))
    }
    pub fn poll(&self) -> anyhow::Result<()> {
        self.send(AppEvent::ControlFlow(ControlFlow::Poll))
    }
    pub fn wait_util(&self, instant: std::time::Instant) -> anyhow::Result<()> {
        self.send(AppEvent::ControlFlow(ControlFlow::WaitUntil(instant)))
    }
}
