pub use super::Engine;
use crate::script::run_frame_fn;
use fool_graphics::canvas::Scene;
use fool_window::WinEvent;
use winit::event::WindowEvent;
impl Engine {
    pub fn run_frame(&mut self) {
        let resource = self.resource.clone();
        let events = &self.events_current_frame;
        if let (Some(render), Some(lua_window), Some(lua_gui)) =
            (&mut self.render, &mut self.lua_window, &mut self.lua_gui)
        {
            render.begin_frame();
            if let Err(err) = run_frame_fn(&self.script, lua_gui, lua_window, events) {
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
    pub fn event(&mut self, event: &WinEvent, raw_event: &WindowEvent) {
        if let Some(render) = &mut self.render {
            render.gui_event(&raw_event);
        }
        self.events_current_frame.push(event.clone());
        if !event.must_redraw() {
            return;
        }
        if let Err(err) = self.script_scheduler.wait_all() {
            log::error!("run lua script_scheduler failed: {}", err);
            self.stop();
            return;
        }
        self.run_frame();
        self.script_scheduler.run();
        self.events_current_frame.clear();
    }
}
