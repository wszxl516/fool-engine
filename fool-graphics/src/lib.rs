pub mod canvas;
pub mod gui;
pub mod render;
pub mod scheduler;
pub mod test;
use crate::render::FrameContext;
use gui::EguiRenderer;
use render::VelloRender;
pub use scheduler::Scheduler;
use std::sync::Arc;
use winit::event::WindowEvent;
use winit::window::Window;
pub struct GraphRender {
    vello: VelloRender,
    egui: EguiRenderer,
    frame: Option<FrameContext>,
}

impl GraphRender {
    pub fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let vello = VelloRender::new(window.clone())?;
        let egui = EguiRenderer::new(
            &vello.device_handle().device,
            vello.format(),
            None,
            window.clone(),
        );
        Ok(Self {
            vello,
            egui,
            frame: None,
        })
    }

    pub fn draw_scene(&mut self, scene: &vello::Scene) {
        self.vello.draw_scene(scene);
    }

    pub fn begin_frame(&mut self) {
        self.frame.replace(self.vello.begin_frame());
        self.egui.begin_frame()
    }
    pub fn end_frame(&mut self) -> anyhow::Result<()> {
        if let Some(mut frame_ctx) = self.frame.take() {
            self.egui.end_frame(&mut frame_ctx);
            self.vello.end_frame(frame_ctx);
            Ok(())
        } else {
            Err(anyhow::anyhow!("call begin_frame first!"))
        }
    }
    pub fn gui_context(&self) -> &egui::Context {
        self.egui.context()
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        self.vello.resize(w, h);
        self.egui.resize(w, h);
    }
    pub fn gui_event(&mut self, event: &WindowEvent) {
        self.egui.handle_event(event);
    }
}
