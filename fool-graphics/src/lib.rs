pub mod canvas;
pub mod gui;
pub mod render;
pub mod test;
use gui::EguiRenderer;
use render::VelloRender;
use std::sync::Arc;
use winit::event::WindowEvent;
use winit::window::Window;
pub struct GraphRender {
    vello: VelloRender,
    egui: EguiRenderer,
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
        Ok(Self { vello, egui })
    }
    pub fn clear_view(&mut self) {
        self.vello.scene_mut().clear();
    }
    pub fn graphics(&mut self, graph_fn: impl FnOnce(&mut vello::Scene)) {
        let scene = self.vello.scene_mut();
        graph_fn(scene.scene());
    }
    pub fn gui(&mut self, ui_fn: impl FnOnce(&egui::Context)) {
        self.vello.render(|encoder, dev, view| {
            self.egui
                .run(&dev.device, &dev.queue, encoder, view, |ctx| ui_fn(ctx));
        });
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
