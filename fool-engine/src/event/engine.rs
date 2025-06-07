use serde::{Deserialize, Serialize};
use std::sync::Arc;
use winit::event_loop::EventLoopProxy;
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum EngineEvent {
    #[default]
    None,
    LoadCursor(String),
    ExitWindow,
}

#[derive(Debug, Clone)]
pub struct EngineEventLoop {
    proxy: Arc<EventLoopProxy<EngineEvent>>,
}
impl EngineEventLoop {
    pub fn new(proxy: EventLoopProxy<EngineEvent>) -> Self {
        Self {
            proxy: Arc::new(proxy),
        }
    }
    pub fn send_event(&self, event: EngineEvent) {
        if let Err(err) = self.proxy.send_event(event.clone()) {
            log::error!("EngineEventLoop send_event {:?}, failed: {}", event, err)
        }
    }
    pub fn load_cursor(&self, name: String) {
        self.send_event(EngineEvent::LoadCursor(name));
    }
    pub fn exit_window(&self) {
        self.send_event(EngineEvent::ExitWindow);
    }
}
