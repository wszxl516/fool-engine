pub use super::Engine;
use crate::lua::{run_event_fn, run_update_fn, run_view_fn};
use fool_graphics::canvas::Scene;
use winit::event_loop::ActiveEventLoop;
impl Engine {
    pub fn view(&mut self, _event_loop: &ActiveEventLoop) {
        let resource = self.resource.clone();
        if let (Some(render), Some(window)) = (&mut self.render, &self.window) {
            let egui_ctx = render.begin_frame();
            if let Err(err) = run_view_fn(
                &self.script.lua,
                egui_ctx.clone(),
                resource.clone(),
                window.clone(),
                self.engine_event_loop.clone(),
            ) {
                log::error!("run lua view failed: {}", err);
                self.stop();
                return;
            }
            let graph = resource.scene.read();
            let mut scene = Scene::new();
            graph.draw(&mut scene);
            render.draw_scene(&scene);
            if let Err(err) = render.end_frame() {
                log::error!("end_frame failed: {}", err);
                self.stop();
            }
        }
    }
    pub fn update(&mut self, _event_loop: &ActiveEventLoop) {
        self.script_scheduler.run();
        if let Err(err) = run_update_fn(&self.script.lua) {
            log::error!("run lua update failed: {}", err);
            self.stop();
        }
        if let Err(err) = self.script_scheduler.wait_all() {
            log::error!("run lua script_scheduler failed: {}", err);
            self.stop();
        }
    }
    pub fn event(&mut self, event: &winit::event::WindowEvent) {
        self.event_state.handle_event(event);
        if let Some(render) = &mut self.render {
            render.gui_event(&event);
        }
        if let Some(window) = &self.window {
            let resource = self.resource.clone();
            if let Err(err) = run_event_fn(
                &self.script.lua,
                &mut self.event_state,
                window.clone(),
                resource,
                self.engine_event_loop.clone(),
            ) {
                log::error!("run lua event failed: {}", err);
                self.stop();
                return;
            }
        }
    }
}
