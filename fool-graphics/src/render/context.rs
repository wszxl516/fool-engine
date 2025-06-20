use pollster::FutureExt;
use std::{num::NonZero, sync::Arc};
use vello::{
    AaSupport, Renderer, RendererOptions,
    util::{DeviceHandle, RenderContext, RenderSurface},
};
use wgpu::{TextureFormat, TextureUsages};
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
        let mut surface = context
            .create_surface(
                window.clone(),
                size.width,
                size.height,
                wgpu::PresentMode::Fifo,
            )
            .block_on()?;
        let surface_format = surface.config.format;
        let copyable_config = wgpu::SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_SRC
                | TextureUsages::COPY_DST,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface.config.alpha_mode,
            view_formats: vec![surface_format],
            desired_maximum_frame_latency: 2,
        };
        surface.config = copyable_config;
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
    pub fn surface(&self) -> &wgpu::Surface<'_> {
        &self.surface.surface
    }
}
