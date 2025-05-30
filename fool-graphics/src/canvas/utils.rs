use image::GenericImageView;
use kurbo::{BezPath, Point};
use vello::peniko::{Blob, Image as VelloImage, ImageFormat};

pub fn add_circle_to_path(path: &mut BezPath, center: Point, radius: f64) {
    const KAPPA: f64 = 0.552284749831;
    let x = center.x;
    let y = center.y;
    let r = radius;
    let c = KAPPA * r;

    path.move_to(Point::new(x + r, y));
    path.curve_to(
        Point::new(x + r, y + c),
        Point::new(x + c, y + r),
        Point::new(x, y + r),
    );
    path.curve_to(
        Point::new(x - c, y + r),
        Point::new(x - r, y + c),
        Point::new(x - r, y),
    );
    path.curve_to(
        Point::new(x - r, y - c),
        Point::new(x - c, y - r),
        Point::new(x, y - r),
    );
    path.curve_to(
        Point::new(x + c, y - r),
        Point::new(x + r, y - c),
        Point::new(x + r, y),
    );
    path.close_path();
}

pub fn load_image_from_file(path: &str) -> VelloImage {
    let img = image::open(path).expect("Failed to open image");
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();
    VelloImage {
        width,
        height,
        data: Blob::from(rgba.into_vec()),
        format: ImageFormat::Rgba8,
        x_extend: Default::default(),
        y_extend: Default::default(),
        quality: Default::default(),
        alpha: 1.0,
    }
}

#[macro_export]
macro_rules! graph_vec2 {
    ($x: expr, $y: expr) => {
        vello::kurbo::Vec2::new($x, $y)
    };
}

#[macro_export]
macro_rules! graph_pt2 {
    ($x: expr, $y: expr) => {
        vello::kurbo::Point::new($x, $y)
    };
}
