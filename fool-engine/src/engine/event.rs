use super::Engine;
use fool_window::{Application, CustomEvent, EventProxy, WinEvent};
use std::{path::PathBuf, sync::Arc};
use winit::{event::WindowEvent, window::Window};
impl Engine {
    fn window_event(&mut self, event: &WinEvent, raw_event: &WindowEvent) {
        if !self.scheduler.running {
            return;
        }
        if event.close_requested() && self.lua_exit_callback() {
            if let Some(proxy) = &self.proxy {
                let _ = proxy.exit();
            }
        }
        if let Some(size) = event.window_resized() {
            if let (Some(render), Some(window), Some(lua_engine)) =
                (&mut self.render, &self.window, &mut self.lua_engine)
            {
                log::trace!("resize render graph to {:?}", size);
                render.resize(size.width, size.height);
                lua_engine.resize(size.width, size.height);
                self.scene_graph
                    .write()
                    .center_with_screen_size(size.width as f64, size.height as f64);
                window.request_redraw();
            }
        }
        self.event(event, raw_event);
    }
}

impl Application for Engine {
    fn init(&mut self, window: Arc<Window>, proxy: &EventProxy) {
        if let Err(err) = self.init(window, proxy) {
            log::error!("init engine failed: {}", err);
            self.stop();
        }
    }
    fn event(&mut self, event: &WinEvent, raw_event: &WindowEvent) {
        if !self.scheduler.running {
            return;
        }
        self.window_event(event, raw_event);
    }
    fn update(&mut self) {
        if !self.scheduler.running {
            return;
        }
        if let (Some(proxy), Some(window)) = (&self.proxy, &self.window) {
            if self.scheduler.trigger_redraw(proxy) {
                window.request_redraw();
            }
        }
    }
    fn exiting(&mut self) {
        self.exiting();
    }
    fn user_event(&mut self, event: Box<dyn CustomEvent>) {
        if let Ok(event) = event.downcast::<EngineEvent>() {
            match *event {
                EngineEvent::Capture(p) => {
                    let full_path = self.base_config.capture_path.clone().join(p);
                    log::trace!("Capture current screen to {}", full_path.display());
                    self.frame_capture.push_back(full_path);
                }
                EngineEvent::FPS(fps) => {
                    log::trace!("set current fps to {}", fps);
                    self.scheduler.set_fps(fps)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EngineEvent {
    Capture(PathBuf),
    FPS(u32),
}
