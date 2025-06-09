pub use super::Engine;
use crate::script::run_frame_fn;
use fool_graphics::canvas::Scene;
use fool_window::WinEvent;
use winit::event::WindowEvent;
impl Engine {
    pub fn run_frame(&mut self) {
        let resource = self.resource.clone();
        let events = &self.events_current_frame;
        if let (Some(render), Some(lua_engine)) = (&mut self.render, &mut self.lua_engine) {
            render.begin_frame();
            if let Err(err) = run_frame_fn(&self.script, lua_engine, events) {
                log::error!("run lua run_frame failed: {}", err);
                self.stop();
                return;
            }
            let graph = resource.scene_graph.read();
            let mut scene = Scene::new();
            graph.draw(&mut scene);
            render.draw_scene(&scene);
            if let Err(err) = render.end_frame(self.frame_capture.pop_front()) {
                log::error!("end_frame failed: {}", err);
                self.stop();
            }
        }
    }
    pub fn event(&mut self, event: &WinEvent, raw_event: &WindowEvent) {
        if let Some(render) = &mut self.render {
            render.gui_event(&raw_event);
        }
        self.events_current_frame.push(event.clone());
        if !event.must_redraw() {
            return;
        }
        self.run_frame();
        if let Err(err) = self.script_scheduler.run() {
            log::error!("run lua script_scheduler failed: {}", err);
            self.stop();
            return;
        }
        self.events_current_frame.clear();
        log::debug!(
            "Frame: {}, elapsed: {:?}",
            self.scheduler.frame_id,
            self.scheduler.frame_id.elapsed()
        );
        self.scheduler.frame_id.reset_timer();
    }
}
