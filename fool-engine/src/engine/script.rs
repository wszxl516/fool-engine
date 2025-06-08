pub use super::Engine;
use crate::script::{run_event_fn, run_update_fn, run_view_fn};
use fool_graphics::canvas::Scene;
use fool_window::WinEvent;
use winit::event::WindowEvent;
impl Engine {
    pub fn view(&mut self) {
        let resource = self.resource.clone();
        if let (Some(render), Some(window), Some(proxy)) =
            (&mut self.render, &self.window, &self.proxy)
        {
            let egui_ctx = render.begin_frame();
            if let Err(err) = run_view_fn(
                &self.script,
                egui_ctx.clone(),
                resource.clone(),
                window.clone(),
                proxy.clone(),
            ) {
                log::error!("run lua view failed: {}", err);
                self.stop();
                return;
            }
            let graph = resource.scene_graph.read();
            let mut scene = Scene::new();
            graph.draw(&mut scene);
            render.draw_scene(&scene);
            if let Err(err) = render.end_frame() {
                log::error!("end_frame failed: {}", err);
                self.stop();
            }
        }
    }
    pub fn update(&mut self) {
        self.script_scheduler.run();
        if let Err(err) = run_update_fn(&self.script) {
            log::error!("run lua update failed: {}", err);
            self.stop();
        }
        if let Err(err) = self.script_scheduler.wait_all() {
            log::error!("run lua script_scheduler failed: {}", err);
            self.stop();
        }
    }
    pub fn event(&mut self, event: &WinEvent, raw_event: &WindowEvent) {
        if let Some(render) = &mut self.render {
            render.gui_event(&raw_event);
        }
        if let (Some(window), Some(proxy)) = (&self.window, &self.proxy) {
            let resource = self.resource.clone();
            if let Err(err) = run_event_fn(
                &self.script,
                &event,
                window.clone(),
                resource,
                proxy.clone(),
            ) {
                log::error!("run lua event failed: {}", err);
                self.stop();
                return;
            }
        }
    }
}
