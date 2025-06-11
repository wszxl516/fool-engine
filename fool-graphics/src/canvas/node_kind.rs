use crate::canvas::style::{CustomBrush, CustomGradient, SimpleColor};

use super::utils::add_circle_to_path;
use super::{Drawable, ImageDrawable, Style, TextDrawable};
use kurbo::{
    Arc, BezPath, CubicBez, Ellipse, Line, PathEl, Point, QuadBez, Rect, RoundedRect,
    RoundedRectRadii, Size, Triangle, Vec2,
};
use peniko::Image;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SceneNodeKind {
    Ellipse {
        center: Point,
        radii: Vec2,
        rotation: f64,
    },
    Circle {
        center: Point,
        radius: f64,
        rotation: f64,
    },
    Line {
        p0: Point,
        p1: Point,
    },
    Rect {
        p0: Point,
        size: Size,
    },
    RoundedRect {
        p0: Point,
        size: Size,
        radii: RoundedRectRadii,
    },
    Triangle {
        a: Point,
        b: Point,
        c: Point,
    },
    QuadBez {
        a: Point,
        b: Point,
        c: Point,
    },
    CubicBez {
        a: Point,
        b: Point,
        c: Point,
        d: Point,
    },
    BezPath {
        elements: Vec<PathEl>,
    },
    Point {
        pos: Point,
    },
    Arc {
        center: Point,
        radii: Vec2,
        start_angle: f64,
        sweep_angle: f64,
        rotation: f64,
    },
    PointLight {
        center: Point,
        radius: f64,
        rotation: f64,
        color: Vec<(f32, SimpleColor)>,
    },
    LightMask {
        screen_size: Size,
        lights: Vec<(Point, f64)>,
        darkness_alpha: u8,
    },
    Text {
        position: Point,
        text: String,
    },
    Image {
        position: Point,
        image: String,
    },
    SpriteImage {
        position: Point,
        image: Image,
    },
}
impl SceneNodeKind {
    pub(crate) fn build(&self, style: &Style) -> BuiltDrawable {
        match self {
            SceneNodeKind::SpriteImage { position, image } => BuiltDrawable::image(
                position.clone(),
                super::VelloImage::Image(image.clone()),
                Default::default(),
            ),
            SceneNodeKind::Image { position, image } => BuiltDrawable::image(
                position.clone(),
                super::VelloImage::Path(image.clone()),
                Default::default(),
            ),
            SceneNodeKind::Text { position, text } => {
                BuiltDrawable::text(*position, text.clone(), style.clone())
            }
            SceneNodeKind::Ellipse {
                center,
                radii,
                rotation,
            } => BuiltDrawable::ellipse(*center, *radii, *rotation, style),

            SceneNodeKind::Circle {
                center,
                radius,
                rotation,
            } => BuiltDrawable::circle(*center, *radius, *rotation, style),

            SceneNodeKind::Line { p0, p1 } => BuiltDrawable::line(*p0, *p1, style),

            SceneNodeKind::Rect { p0, size } => BuiltDrawable::rect(*p0, *size, style),

            SceneNodeKind::RoundedRect { p0, size, radii } => {
                BuiltDrawable::round_rect(*p0, *size, *radii, style)
            }

            SceneNodeKind::Triangle { a, b, c } => BuiltDrawable::triangle(*a, *b, *c, style),

            SceneNodeKind::QuadBez { a, b, c } => BuiltDrawable::quad_bez(*a, *b, *c, style),

            SceneNodeKind::CubicBez { a, b, c, d } => {
                BuiltDrawable::cubic_bez(*a, *b, *c, *d, style)
            }

            SceneNodeKind::BezPath { elements } => BuiltDrawable::bez_path(elements, style),

            SceneNodeKind::Point { pos } => BuiltDrawable::point(*pos, style),

            SceneNodeKind::Arc {
                center,
                radii,
                start_angle,
                sweep_angle,
                rotation,
            } => BuiltDrawable::arc(
                *center,
                *radii,
                *start_angle,
                *sweep_angle,
                *rotation,
                style,
            ),

            SceneNodeKind::PointLight {
                center,
                radius,
                rotation,
                color,
            } => BuiltDrawable::point_light(*center, *radius, *rotation, color),

            SceneNodeKind::LightMask {
                screen_size,
                lights,
                darkness_alpha,
            } => BuiltDrawable::light_mask(*screen_size, lights, *darkness_alpha),
        }
    }
}

pub(crate) struct BuiltDrawable {
    pub(crate) style: Style,
    pub(crate) drawable: Box<dyn Drawable>,
}
impl BuiltDrawable {
    #[inline]
    pub fn image(position: Point, image: super::VelloImage, style: Style) -> Self {
        Self {
            style: style,
            drawable: Box::new(ImageDrawable { position, image }),
        }
    }
    #[inline]
    pub fn text(position: Point, text: String, style: Style) -> Self {
        Self {
            style: Default::default(),
            drawable: Box::new(TextDrawable {
                position,
                text,
                style,
            }),
        }
    }
    #[inline]
    pub fn ellipse(center: Point, radii: Vec2, rotation: f64, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(Ellipse::new(center, radii, rotation)),
        }
    }
    #[inline]
    pub fn circle(center: Point, radii: f64, rotation: f64, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(Ellipse::new(center, Vec2::new(radii, radii), rotation)),
        }
    }
    #[inline]
    pub fn line(p0: Point, p1: Point, style: &Style) -> Self {
        let style = style.clone().with_fill(None);
        Self {
            style,
            drawable: Box::new(Line::new(p0, p1)),
        }
    }
    #[inline]
    pub fn rect(p0: Point, size: Size, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(Rect::from_center_size(p0, size)),
        }
    }
    #[inline]
    pub fn round_rect(p0: Point, size: Size, radii: RoundedRectRadii, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(RoundedRect::from_rect(
                Rect::from_center_size(p0, size),
                radii,
            )),
        }
    }
    #[inline]
    pub fn triangle(a: Point, b: Point, c: Point, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(Triangle::new(a, b, c)),
        }
    }
    #[inline]
    pub fn quad_bez(a: Point, b: Point, c: Point, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(QuadBez::new(a, b, c)),
        }
    }
    #[inline]
    pub fn cubic_bez(a: Point, b: Point, c: Point, d: Point, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(CubicBez::new(a, b, c, d)),
        }
    }
    #[inline]
    pub fn bez_path(p: &Vec<PathEl>, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(BezPath::from_vec(p.clone())),
        }
    }
    #[inline]
    pub fn point(a: Point, style: &Style) -> Self {
        let style = style.clone().with_fill(None);
        Self {
            style,
            drawable: Box::new(Ellipse::new(a, Vec2::new(0.5, 0.5), 0.0)),
        }
    }
    #[inline]
    pub fn arc(
        center: Point,
        radii: Vec2,
        start_angle: f64,
        sweep_angle: f64,
        rotation: f64,
        style: &Style,
    ) -> Self {
        Self {
            style: style.clone(),
            drawable: Box::new(Arc::new(center, radii, start_angle, sweep_angle, rotation)),
        }
    }
    #[inline]
    pub fn point_light(
        center: Point,
        radius: f64,
        rotation: f64,
        color: &Vec<(f32, SimpleColor)>,
    ) -> Self {
        let gradient = CustomGradient {
            kind: super::style::CustomGradientKind::Radial,
            extend: super::style::CustomExtend::Pad,
            colors: color.to_vec(),
        };
        let brush = CustomBrush::Gradient(gradient);
        Self {
            style: Style::default().with_fill(Some(brush)),
            drawable: Box::new(Ellipse::new(center, Vec2::new(radius, radius), rotation)),
        }
    }
    #[inline]
    pub fn light_mask(screen_size: Size, lights: &[(Point, f64)], darkness_alpha: u8) -> Self {
        let mut path = BezPath::new();
        let rect = Rect::from_origin_size(Point::ORIGIN, screen_size);
        path.move_to(rect.origin());
        path.line_to(Point::new(rect.x1, rect.y0));
        path.line_to(Point::new(rect.x1, rect.y1));
        path.line_to(Point::new(rect.x0, rect.y1));
        path.close_path();
        for &(center, radius) in lights {
            add_circle_to_path(&mut path, center, radius);
        }

        let style = Style {
            fill: Some(CustomBrush::Color(SimpleColor {
                r: 0,
                g: 0,
                b: 0,
                a: darkness_alpha,
            })),
            fill_rule: peniko::Fill::EvenOdd,
            ..Default::default()
        };

        Self {
            style,
            drawable: Box::new(path),
        }
    }
}
