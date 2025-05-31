#![allow(dead_code)]
use std::sync::Arc;
use vello::AaConfig;
use vello::{peniko::color::palette, util::DeviceHandle};
use wgpu::CommandEncoder;
use winit::window::Window;
mod context;
mod scene;
use context::ContextRender;
use scene::SceneBuilder;

pub struct VelloRender {
    context: ContextRender,
    scene: SceneBuilder,
}

impl VelloRender {
    pub fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let context = ContextRender::new(window)
            .map_err(|err| anyhow::anyhow!("Failed to create vello surface:{}", err))?;
        Ok(Self {
            context,
            scene: SceneBuilder::new(),
        })
    }
    pub fn render(
        &mut self,
        f: impl FnOnce(&mut CommandEncoder, &DeviceHandle, &wgpu::TextureView),
    ) {
        let scene = self.scene.build();
        let device = &mut self.context;
        let surface = &mut device.surface;
        let device_handle = &device.context.devices[surface.dev_id];
        device
            .renderer
            .render_to_texture(
                &device_handle.device,
                &device_handle.queue,
                &scene,
                &surface.target_view,
                &vello::RenderParams {
                    base_color: palette::css::BLACK,
                    width: surface.config.width,
                    height: surface.config.height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .expect("Render failed");
        let surface_texture = surface
            .surface
            .get_current_texture()
            .expect("Failed to get texture");
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

        f(&mut encoder, device_handle, &final_view);
        device_handle.queue.submit(Some(encoder.finish()));
        surface_texture.present();
        device_handle.device.poll(wgpu::Maintain::Poll);
    }

    pub fn scene_mut(&mut self) -> &mut SceneBuilder {
        &mut self.scene
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
