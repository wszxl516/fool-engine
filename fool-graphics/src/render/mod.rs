#![allow(dead_code)]
use std::path::PathBuf;
use std::sync::Arc;
use vello::AaConfig;
use vello::{Scene, peniko::color::palette, util::DeviceHandle};
use winit::window::Window;
mod capture;
mod context;
mod frame;
use capture::FrameCapture;
use context::ContextRender;
pub use frame::FrameContext;
pub struct VelloRender {
    context: ContextRender,
    frame_buffer: Option<FrameCapture>,
}

impl VelloRender {
    pub fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let context = ContextRender::new(window)
            .map_err(|err| anyhow::anyhow!("Failed to create vello surface:{}", err))?;
        Ok(Self {
            context,
            frame_buffer: None,
        })
    }
    pub fn draw_scene(&mut self, scene: &Scene) -> anyhow::Result<()> {
        let context = &mut self.context;
        let surface = &mut context.surface;
        let device_handle = &context.context.devices[surface.dev_id];
        context.renderer.render_to_texture(
            &device_handle.device,
            &device_handle.queue,
            scene,
            &surface.target_view,
            &vello::RenderParams {
                base_color: palette::css::BLACK,
                width: surface.config.width,
                height: surface.config.height,
                antialiasing_method: AaConfig::Msaa16,
            },
        )?;
        Ok(())
    }
    pub fn begin_frame(&mut self) -> anyhow::Result<FrameContext> {
        let context = &mut self.context;
        let surface = &mut context.surface;
        let device_handle = &context.context.devices[surface.dev_id];
        let surface_texture = surface.surface.get_current_texture()?;

        let final_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            device_handle
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Main Encoder"),
                });
        surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &surface.target_view,
            &final_view,
        );
        Ok(FrameContext {
            encoder,
            device: device_handle.device.clone(),
            queue: device_handle.queue.clone(),
            target_view: final_view,
            surface_texture,
        })
    }
    pub fn end_frame(&mut self, ctx: FrameContext, capture_to: Option<impl Into<PathBuf>>) {
        let mut ctx = ctx;
        if let Some(capture_to) = capture_to {
            let config = &self.context.surface.config;
            let frame = FrameCapture::new(&config, &ctx.device, capture_to.into());
            let _ = frame.copy2buffer(&ctx.surface_texture, &mut ctx.encoder);
            self.frame_buffer.replace(frame);
        }
        ctx.queue.submit(Some(ctx.encoder.finish()));
        ctx.surface_texture.present();
        ctx.device.poll(wgpu::Maintain::Poll);

        if let Some(frame) = self.frame_buffer.take() {
            let device = ctx.device.clone();
            std::thread::spawn(move || {
                frame.finish(&device);
            });
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.context.resize(w, h);
    }

    pub fn device_handle(&self) -> &DeviceHandle {
        self.context.device_handle()
    }
    pub fn format(&self) -> wgpu::TextureFormat {
        self.context.format()
    }
}
