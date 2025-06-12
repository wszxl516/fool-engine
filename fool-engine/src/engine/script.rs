pub use super::Engine;
use crate::script::run_frame_fn;
use fool_graphics::canvas::Scene;
use fool_window::WinEvent;
use winit::event::WindowEvent;
impl Engine {
    pub fn run_frame(&mut self) {
        let scene_graph = self.scene_graph.clone();
        let events = &self.events_current_frame;
        if let (Some(render), Some(lua_engine)) = (&mut self.render, &mut self.lua_engine) {
            render.begin_frame();
            let frame_result = run_frame_fn(&self.script, lua_engine, events);
            let mut graph = scene_graph.write();
            let mut scene = Scene::new();
            let graph_result = graph.draw(&mut scene);
            render.draw_scene(&scene);
            graph.root.clear_children();
            crate::try_or_return!(
                render.end_frame(self.frame_capture.pop_front()),
                "end_frame",
                self.stop()
            );
            // must after current frame end
            crate::try_or_return!(frame_result, "run lua run_frame", self.stop());
            crate::try_or_return!(graph_result, "run lua graph.draw", self.stop());
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
        if let Err(err) = self
            .script_scheduler
            .tick(&self.script, self.scheduler.frame_id.into())
        {
            log::error!("run lua script_scheduler failed: {}", err);
            self.stop();
            return;
        }
        self.events_current_frame.clear();
        log::trace!(
            "Frame: {}, elapsed: {:?}",
            self.scheduler.frame_id,
            self.scheduler.frame_id.elapsed()
        );
    }
}
#[macro_export]
macro_rules! try_or_return {
    ($result:expr, $ctx:expr, $run: expr) => {
        if let Err(err) = $result {
            log::error!("{} failed: {}", $ctx, err);
            $run;
            return;
        }
    };
}
