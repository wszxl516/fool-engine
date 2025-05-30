use std::sync::Arc;

use egui::Context;
use egui_wgpu::wgpu::{CommandEncoder, Device, Queue, StoreOp, TextureFormat, TextureView};
use egui_wgpu::{Renderer, ScreenDescriptor, wgpu};
use egui_winit::State;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct EguiRenderer {
    state: State,
    renderer: Renderer,
    screen_descriptor: ScreenDescriptor,
    window: Arc<Window>,
    need_repaint: bool,
}

impl EguiRenderer {
    pub fn context(&self) -> &Context {
        self.state.egui_ctx()
    }

    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        window: Arc<Window>,
    ) -> EguiRenderer {
        let egui_context = Context::default();

        let egui_state = egui_winit::State::new(
            egui_context,
            egui::viewport::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(2 * 1024),
        );
        let egui_renderer =
            Renderer::new(device, output_color_format, output_depth_format, 1, true);
        let size = window.inner_size();
        EguiRenderer {
            state: egui_state,
            renderer: egui_renderer,
            screen_descriptor: ScreenDescriptor {
                size_in_pixels: [size.width, size.height],
                pixels_per_point: 1.0,
            },
            window: window,
            need_repaint: false,
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.screen_descriptor.size_in_pixels = [width, height];
    }
    pub fn handle_event(&mut self, event: &WindowEvent) {
        let response = self.state.on_window_event(&self.window, event);
        self.need_repaint = response.repaint
    }
    pub fn run(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        f: impl FnOnce(&Context),
    ) {
        let ctx = self.begin_frame();
        f(ctx);
        self.end_frame(device, queue, encoder, view);
    }
    pub fn begin_frame(&mut self) -> &Context {
        let raw_input = self.state.take_egui_input(&self.window);
        let ctx = self.state.egui_ctx();
        ctx.begin_pass(raw_input);
        ctx
    }

    pub fn end_frame(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        view: &TextureView,
    ) {
        self.context()
            .set_pixels_per_point(self.screen_descriptor.pixels_per_point);

        let full_output = self.state.egui_ctx().end_pass();

        self.state
            .handle_platform_output(&self.window, full_output.platform_output);

        let tris = self
            .state
            .egui_ctx()
            .tessellate(full_output.shapes, self.state.egui_ctx().pixels_per_point());
        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }
        self.renderer
            .update_buffers(device, queue, encoder, &tris, &self.screen_descriptor);
        let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: egui_wgpu::wgpu::Operations {
                    load: egui_wgpu::wgpu::LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            label: Some("egui main render pass"),
            occlusion_query_set: None,
        });

        self.renderer
            .render(&mut rpass.forget_lifetime(), &tris, &self.screen_descriptor);
        for x in &full_output.textures_delta.free {
            self.renderer.free_texture(x)
        }
    }
}
