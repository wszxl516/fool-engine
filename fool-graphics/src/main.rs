use anyhow::Result;
use fool_graphics::App;
use fool_graphics::AppModel;
use fool_graphics::graph_vec2;
use winit::dpi::LogicalSize;
use winit::dpi::Size;
use winit::event_loop::EventLoop;
use winit::platform::x11::WindowAttributesExtX11;
use winit::window::Window;
fn main() -> Result<()> {
    let attr = Window::default_attributes()
        .with_base_size(Size::Logical(LogicalSize {
            width: 800.0,
            height: 600.0,
        }))
        .with_resizable(true)
        .with_title("Vello Shapes");
    let mut app = App::new(30, attr, Box::new(TestApp));
    let event_loop = EventLoop::new()?;
    event_loop
        .run_app(&mut app)
        .expect("Couldn't run event loop");
    Ok(())
}
struct TestApp;
impl AppModel for TestApp {
    fn graphics(&mut self, scene: &mut vello::Scene, _window: &winit::window::Window) {
        test_graph(scene);
    }
    fn gui(&mut self, context: &egui::Context, window: &winit::window::Window) {
        test_gui(context, window);
    }
}
pub fn test_gui(ctx: &egui::Context, _window: &winit::window::Window) {
    egui::Window::new("test")
        .default_pos(egui::pos2(400.0, 400.0))
        .fixed_size(egui::vec2(100.0, 100.0))
        .title_bar(false)
        .resizable(true)
        .show(ctx, move |ui| {
            if ui.button("aaa").clicked() {
                println!("button clicked!")
            }
            egui::ComboBox::new(11, "aa").show_ui(ui, |u| {
                let _ = u.selectable_label(false, "aa");
                let _ = u.selectable_label(true, "bb");
                u.selectable_label(false, "cc")
            });
        });
}
pub fn test_graph(sc: &mut vello::Scene) {
    use fool_graphics::canvas::StokeStyle;
    use fool_graphics::canvas::{SceneGraph, SceneNode, Style, load_image_from_file};
    use kurbo::{PathEl, Point, RoundedRectRadii, Vec2};
    use kurbo::{Size, Stroke};
    use peniko::Color;

    let mut root = SceneNode::empty();
    root.set_style(&Style::default().with_opacity(0.8));

    let red = Style::default().with_fill(Some(Color::from_rgba8(255, 0, 0, 250)));
    let green = Style::default().with_fill(Some(Color::from_rgba8(0, 255, 0, 200)));
    let blue = Style::default().with_fill(Some(Color::from_rgba8(0, 0, 255, 150)));
    let semi = Style::default().with_fill(Some(Color::from_rgba8(100, 100, 10, 100)));
    let yellow = Style::default().with_fill(Some(Color::from_rgba8(255, 255, 0, 50)));

    root.add_child(&SceneNode::circle(
        Point::new(100.0, 100.0),
        30.0,
        0.0,
        &red,
    ));
    root.add_child(&SceneNode::ellipse(
        Point::new(200.0, 100.0),
        Vec2::new(50.0, 30.0),
        0.0,
        &green,
    ));
    root.add_child(&SceneNode::line(
        Point::new(50.0, 200.0),
        Point::new(150.0, 200.0),
        &blue,
    ));
    root.add_child(&SceneNode::rect(
        Point::new(250.0, 50.0),
        Point::new(320.0, 120.0),
        &blue,
    ));
    root.add_child(&SceneNode::round_rect(
        Point::new(350.0, 50.0),
        Point::new(420.0, 120.0),
        RoundedRectRadii::from_single_radius(10.0),
        &semi,
    ));
    root.add_child(&SceneNode::triangle(
        Point::new(100.0, 300.0),
        Point::new(130.0, 350.0),
        Point::new(70.0, 350.0),
        &red,
    ));
    root.add_child(&SceneNode::quad_bez(
        Point::new(200.0, 300.0),
        Point::new(250.0, 250.0),
        Point::new(300.0, 300.0),
        &green,
    ));
    root.add_child(&SceneNode::cubic_bez(
        Point::new(320.0, 300.0),
        Point::new(340.0, 250.0),
        Point::new(380.0, 350.0),
        Point::new(400.0, 300.0),
        &blue,
    ));
    root.add_child(&SceneNode::bez_path(
        vec![
            PathEl::MoveTo(Point::new(100.0, 100.0)),
            PathEl::LineTo(Point::new(200.0, 250.0)),
            PathEl::QuadTo(Point::new(500.0, 500.0), Point::new(550.0, 250.0)),
            PathEl::ClosePath,
        ],
        &yellow,
    ));
    root.add_child(&SceneNode::arc(
        Point::new(150.0, 400.0),
        Vec2::new(50.0, 30.0),
        0.0,
        std::f64::consts::PI,
        0.0,
        &green,
    ));
    let point_fill = red.clone().with_stoke(Some(StokeStyle {
        stroke: Stroke::new(1.0),
        brush: Color::from_rgba8(255, 0, 255, 255).into(),
    }));
    let mut p = SceneNode::point(Point::new(500.0, 500.0), &point_fill);
    p.set_style(&point_fill);
    root.add_child(&p);
    root.add_child(&SceneNode::point_light(
        Point::new(400.0, 300.0),
        100.0,
        0.0,
        0.3,
        Color::from_rgb8(255, 255, 255),
    ));
    root.add_child(&SceneNode::point_light(
        Point::new(300.0, 300.0),
        100.0,
        0.0,
        0.3,
        Color::from_rgb8(255, 255, 255),
    ));
    root.add_child(&SceneNode::light_mask(
        Size::new(800.0, 600.0),
        &vec![
            (Point::new(400.0, 300.0), 100.0),
            (Point::new(300.0, 300.0), 100.0),
        ],
        100,
    ));
    let img = load_image_from_file("./fool-graphics/linux.png");
    root.add_child(&SceneNode::image(img, &point_fill));
    root.add_child(&SceneNode::text(
        graph_vec2!(0.0, 0.0),
        "Hello!".to_string(),
        fool_graphics::canvas::Style::default()
            .with_align(Some(fool_graphics::canvas::TextAlign::Left))
            .with_fill(Some(Color::from_rgba8(255, 0, 0, 50)))
            .with_font_size(Some(22.0)),
    ));
    let graph = SceneGraph {
        root,
        font_mgr: Default::default(),
    };
    graph.draw(sc);
}
