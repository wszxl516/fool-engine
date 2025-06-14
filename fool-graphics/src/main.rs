use fool_graphics::canvas::{Animation, SceneGraph, SceneNode, Sprite, Style};
use fool_graphics::{GraphRender, Scheduler, graph_pt2, graph_vec2};
use fool_resource::Resource;
use kurbo::Affine;
use peniko::Image;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::WindowEvent,
    event_loop::EventLoopBuilder,
    event_loop::{ActiveEventLoop, ControlFlow},
    platform::x11::{EventLoopBuilderExtX11, WindowAttributesExtX11},
    window::{Window, WindowAttributes},
};
fn main() -> anyhow::Result<()> {
    let window_attr = Window::default_attributes()
        .with_base_size(Size::Logical(LogicalSize {
            width: 800.0,
            height: 600.0,
        }))
        .with_resizable(true)
        .with_title("Test Engine");
    let event_loop = EventLoopBuilder::<()>::default()
        .with_x11()
        .with_any_thread(true)
        .build()?;
    let mut engine = Engine::new(30, window_attr)?;
    event_loop
        .run_app(&mut engine)
        .expect("Couldn't run event loop");
    Ok(())
}

pub struct Engine {
    window_attr: WindowAttributes,
    window: Option<Arc<Window>>,
    render: Option<GraphRender>,
    #[allow(dead_code)]
    sprite: Sprite<usize>,
    animation: Animation,
    scheduler: Scheduler,
    img: Resource<String, Arc<Image>>,
    x: f64,
    y: f64,
}
impl Engine {
    pub fn new(fps: u32, window_attr: WindowAttributes) -> anyhow::Result<Self> {
        let img = Resource::default();

        let sprite_img = image::open("./fool-graphics/player.png").expect("Failed to open image");
        let mut sprite = Sprite::from_image(Arc::new(sprite_img), 64, 64, 0usize..32);
        sprite.create_animation("run", 24..32, 5)?;
        let animation = sprite.get_animation("run").unwrap();
        Ok(Engine {
            window: None,
            window_attr,
            render: None,
            sprite,
            animation,
            scheduler: Scheduler::new(fps),
            img,
            x: 0.0,
            y: 64.0,
        })
    }

    pub fn init(
        &mut self,
        window: Arc<Window>,
        _event_loop: &ActiveEventLoop,
    ) -> anyhow::Result<()> {
        self.window.replace(window.clone());
        let render = GraphRender::new(window.clone())?;
        self.render.replace(render);

        Ok(())
    }
    pub fn draw(&mut self) {
        if let Some(render) = &mut self.render {
            render.begin_frame();
            let mut scene = vello::Scene::new();
            let mut root = SceneNode::empty();
            println!("current: {}", self.animation.current());
            self.animation.next();
            let node = self.animation.to_node(self.x, self.y);
            root.add_child(&node);
            root.set_style(
                &Style::default()
                    .with_opacity(0.8)
                    .with_translation(Affine::translate(graph_vec2!(100.0, 100.0))),
            );
            root.add_child(&SceneNode::circle(
                graph_pt2!(300.0, 300.0),
                50.0,
                0.0,
                &Default::default(),
            ));
            let sgraph = SceneGraph {
                root: root,
                style: Default::default(),
                font_mgr: Default::default(),
                img_mgr: self.img.clone(),
            };
            sgraph.draw(&mut scene).unwrap();
            render.draw_scene(&scene);
            render.end_frame(None::<&str>).unwrap();
            self.x += 1.0;
            if self.x > 800.0 {
                self.x = 0.0
            }
        }
    }
    fn window_event(&mut self, event: &winit::event::WindowEvent, _event_loop: &ActiveEventLoop) {
        match event {
            WindowEvent::Resized(size) => {
                if let (Some(render), Some(window)) = (&mut self.render, &self.window) {
                    render.resize(size.width, size.height);
                    window.request_redraw();
                }
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                _event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            _ => {}
        }
    }
}

impl ApplicationHandler<()> for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            match event_loop.create_window(self.window_attr.clone()) {
                Ok(window) => {
                    let window = Arc::new(window);
                    if let Err(err) = self.init(window.clone(), event_loop) {
                        log::error!("init engine failed: {}", err);
                    }
                }
                Err(err) => {
                    log::error!("create_window failed: {}", err);
                }
            }
        }
        event_loop.set_control_flow(ControlFlow::Wait);
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.scheduler.trigger_redraw(event_loop) {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.window_event(&event, event_loop);
    }
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let (Some(render), Some(window)) = (self.render.take(), self.window.take()) {
            drop(window);
            drop(render);
        }
        log::debug!("exiting window");
    }
}
