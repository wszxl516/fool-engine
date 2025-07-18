pub use super::Engine;
use crate::{
    engine::EngineStatus,
    script::{exit_fn, pause_fn, run_fn},
};
use fool_graphics::canvas::Scene;
use fool_window::WinEvent;
use winit::event::WindowEvent;
impl Engine {
    pub fn run_frame(&mut self) {
        let scene_graph = self.scene_graph.clone();
        let events = &self.events_current_frame;
        if let (Some(render), Some(lua_engine)) = (&mut self.render, &mut self.lua_engine) {
            crate::try_or_return!(render.begin_frame(), "begin_frame", self.stop());

            let status = { self.status.read().clone() };
            let frame_result = match status {
                EngineStatus::Pause => pause_fn(&self.script, lua_engine, events),
                EngineStatus::Exiting => exit_fn(&self.script, lua_engine, events),
                _ => run_fn(&self.script, lua_engine, events),
            };
            let mut graph = scene_graph.write();
            let mut scene = Scene::new();
            let graph_result = graph.draw(&mut scene);
            let scene_result = render.draw_scene(&scene);
            graph.reset();
            crate::try_or_return!(
                render.end_frame(self.frame_capture.pop_front()),
                "end_frame",
                self.stop()
            );
            // must after current frame end
            crate::try_or_return!(frame_result, "run lua run_frame", self.stop());
            crate::try_or_return!(graph_result, "run lua graph.draw", self.stop());
            crate::try_or_return!(scene_result, "run lua draw_scene", self.stop());
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
        if let Err(err) = self
            .script_scheduler
            .fetch_result(&self.script, self.scheduler.frame_id.into())
        {
            log::error!("run lua script_scheduler failed: {}", err);
            self.stop();
            return;
        }
        self.run_frame();
        self.script_scheduler
            .start_update(self.scheduler.frame_id.into());
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
