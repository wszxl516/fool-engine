use pollster::FutureExt;
use std::{num::NonZero, sync::Arc};
use vello::{
    AaSupport, Renderer, RendererOptions,
    util::{DeviceHandle, RenderContext, RenderSurface},
};
use wgpu::TextureFormat;
use winit::window::Window;

pub struct ContextRender {
    pub context: RenderContext,
    pub renderer: Renderer,
    pub surface: Box<RenderSurface<'static>>,
}

impl ContextRender {
    pub fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let mut context = RenderContext::new();
        let size = window.inner_size();
        let surface = context
            .create_surface(
                window.clone(),
                size.width,
                size.height,
                wgpu::PresentMode::Fifo,
            )
            .block_on()?;
        let renderer = Renderer::new(
            &context.devices[surface.dev_id].device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: AaSupport::all(),
                num_init_threads: NonZero::new(1),
                pipeline_cache: None,
            },
        )
        .expect("Failed to create renderer");
        Ok(Self {
            context,
            renderer,
            surface: Box::new(surface),
        })
    }
    pub fn format(&self) -> TextureFormat {
        self.surface.format
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.context
            .resize_surface(&mut self.surface, width, height);
    }
    pub fn device_handle(&self) -> &DeviceHandle {
        &self.context.devices[self.surface.dev_id]
    }
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface.surface
    }
}
