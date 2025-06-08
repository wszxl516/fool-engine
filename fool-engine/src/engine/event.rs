use super::Engine;
use fool_window::{Application, CustomEvent, EventProxy, WinEvent};
use std::sync::Arc;
use winit::{event::WindowEvent, window::Window};
impl Engine {
    fn window_event(&mut self, event: &WinEvent, raw_event: &WindowEvent) {
        if !self.scheduler.running {
            return;
        }
        self.event(event, raw_event);
        match raw_event {
            WindowEvent::Resized(size) => {
                if let (Some(render), Some(window)) = (&mut self.render, &self.window) {
                    render.resize(size.width, size.height);
                    self.resource
                        .scene_graph
                        .write()
                        .center_with_screen_size(size.width as f64, size.height as f64);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                self.update();
                self.view();
            }
            _ => {}
        }
    }
}

impl Application for Engine {
    fn init(&mut self, window: Arc<Window>, proxy: &EventProxy) {
        if let Err(err) = self.init(window, proxy) {
            log::error!("init engine failed: {}", err);
            self.stop();
        }
    }
    fn event(&mut self, event: &WinEvent, raw_event: &WindowEvent, _proxy: &EventProxy) {
        self.window_event(event, raw_event);
    }
    fn update(&mut self, _proxy: &EventProxy) {
        if let (Some(proxy), Some(window)) = (&self.proxy, &self.window) {
            if self.scheduler.trigger_redraw(proxy) {
                window.request_redraw();
            }
        }
    }
    fn user_event(&mut self, _event: Box<dyn CustomEvent>) {
        println!("user_event")
    }
    fn exiting(&mut self) {
        self.exiting();
    }
}
