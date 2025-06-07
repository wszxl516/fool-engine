use super::Engine;
use crate::event::EngineEvent;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
};
impl Engine {
    fn window_event(&mut self, event: &winit::event::WindowEvent, event_loop: &ActiveEventLoop) {
        if !self.scheduler.running {
            return;
        }
        self.event(event);
        match event {
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
                self.update(event_loop);
                self.view(event_loop);
            }
            _ => {}
        }
    }
    fn engine_event(&mut self, event_loop: &ActiveEventLoop, event: EngineEvent) {
        match event {
            EngineEvent::LoadCursor(cursor) => {
                if let Err(err) = self.resource.preload_cursor(&cursor, event_loop) {
                    log::error!("load_cursor {} failed: {}", cursor, err)
                }
            }
            EngineEvent::ExitWindow => {
                log::debug!("exit window");
                event_loop.exit()
            }
            _ => {}
        }
    }
}

impl ApplicationHandler<EngineEvent> for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            match event_loop.create_window(self.window_attr.clone()) {
                Ok(window) => {
                    let window = Arc::new(window);
                    if let Err(err) = self.init(window.clone(), event_loop) {
                        log::error!("init engine failed: {}", err);
                        self.stop();
                    }
                }
                Err(err) => {
                    log::error!("create_window failed: {}", err);
                    self.stop();
                }
            }
        }
        event_loop.set_control_flow(ControlFlow::Wait);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.window_event(&event, event_loop);
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
