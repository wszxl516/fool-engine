use gui::EguiRenderer;
use render::Render;
use scheduler::Scheduler;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::window::Window;
use winit::window::WindowAttributes;

pub mod canvas;
mod gui;
mod render;
mod scheduler;
pub struct App {
    render: Render,
    pub window: Option<Arc<Window>>,
    pub window_attr: WindowAttributes,
    egui_render: Option<EguiRenderer>,
    pub scheduler: Scheduler,
    model: Box<dyn AppModel>,
}

impl App {
    pub fn new(fps: u32, window_attr: WindowAttributes, model: Box<dyn AppModel>) -> Self {
        Self {
            render: Render::new(),
            window: None,
            egui_render: None,
            scheduler: Scheduler::new(fps),
            window_attr,
            model,
        }
    }
    pub fn init(
        &mut self,
        window: Arc<Window>,
        event_loop: &ActiveEventLoop,
    ) -> anyhow::Result<()> {
        self.window.replace(window.clone());
        self.render.init(window.clone())?;
        if let Some(r) = &self.render.context {
            self.egui_render.replace(EguiRenderer::new(
                &r.device_handle().device,
                r.format(),
                None,
                window,
            ));
        }
        if let (Some(egui), Some(win)) = (&self.egui_render, &self.window) {
            self.model.init(egui.context(), win.clone());
        }
        self.model.event_loop(event_loop);
        Ok(())
    }
    pub fn create_window(
        event_loop: &ActiveEventLoop,
        window_attr: &WindowAttributes,
    ) -> Arc<Window> {
        Arc::new(event_loop.create_window(window_attr.clone()).unwrap())
    }
    pub fn trigger_redraw(&mut self, event_loop: &ActiveEventLoop) {
        let now = std::time::Instant::now();
        if now >= self.scheduler.next_frame_time {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
            self.scheduler.advance();
        }
        let next = self.scheduler.next_frame_time;
        let wait = if next > now {
            next
        } else {
            now + std::time::Duration::from_millis(1)
        };
        event_loop.set_control_flow(ControlFlow::WaitUntil(wait));
    }
    pub fn size(&self) -> PhysicalSize<u32> {
        self.window
            .as_ref()
            .map(|w| w.inner_size())
            .unwrap_or_else(|| PhysicalSize::from_logical(LogicalSize::new(800.0, 800.0), 1.0))
    }

    pub fn view(&mut self) {
        let scene = self.render.scene_mut();
        scene.clear();
        if let Some(window) = &self.window {
            self.model.graphics(scene.scene(), window.clone());
            if let Some(egui) = &mut self.egui_render {
                self.render.render(|encoder, dev, view| {
                    egui.run(&dev.device, &dev.queue, encoder, view, |ctx| {
                        self.model.gui(ctx, window.clone())
                    });
                });
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Self::create_window(event_loop, &self.window_attr);
            self.init(window.clone(), event_loop)
                .expect("app init failed");
        }
        event_loop.set_control_flow(ControlFlow::Wait);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.model.window_event(&event);
        if let Some(egui) = &mut self.egui_render {
            egui.handle_event(&event);
        }
        match event {
            WindowEvent::CloseRequested => {
                if let Some(egui) = self.egui_render.take() {
                    drop(egui);
                }
                event_loop.exit();
                std::process::exit(0)
            }
            WindowEvent::Resized(size) => {
                self.render.resize(size.width, size.height);
                if let Some(egui) = &mut self.egui_render {
                    egui.resize(size.width, size.height);
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => self.view(),
            _ => {}
        }
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.trigger_redraw(event_loop);
        self.model.event_loop(event_loop);
    }
}

pub trait AppModel {
    fn init(&mut self, _context: &egui::Context, _window: Arc<winit::window::Window>) {}
    fn graphics(&mut self, _scene: &mut vello::Scene, _window: Arc<winit::window::Window>) {}
    fn gui(&mut self, _context: &egui::Context, _window: Arc<winit::window::Window>) {}
    fn window_event(&mut self, _event: &WindowEvent) {}
    fn event_loop(&mut self, _event_loop: &ActiveEventLoop) {}
}
