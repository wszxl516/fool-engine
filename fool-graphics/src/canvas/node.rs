use crate::canvas::style::SimpleColor;

use super::{SceneNodeKind, Style};
use kurbo::{PathEl, Point, RoundedRectRadii, Size, Vec2};
use serde::{Deserialize, Serialize};

const fn default_apply_parent_style() -> bool {
    true
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SceneNode {
    #[serde(default)]
    pub style: Style,
    #[serde(default)]
    pub drawable: Option<SceneNodeKind>,
    #[serde(default = "default_apply_parent_style")]
    pub apply_parent_style: bool,
    #[serde(default)]
    pub children: Vec<SceneNode>,
}
impl SceneNode {
    pub fn new(d: SceneNodeKind, style: &Style) -> Self {
        Self {
            style: style.clone(),
            drawable: Some(d),
            children: Default::default(),
            apply_parent_style: true,
        }
    }
    pub fn empty() -> Self {
        Self::default()
    }
}
impl SceneNode {
    pub fn image(position: Point, image: String) -> Self {
        Self::new(
            SceneNodeKind::Image { position, image },
            &Default::default(),
        )
    }
    pub fn text(position: Point, text: String, style: Style) -> Self {
        Self::new(SceneNodeKind::Text { position, text }, &style)
    }
    pub fn ellipse(center: Point, radii: Vec2, rotation: f64, style: &Style) -> Self {
        Self::new(
            SceneNodeKind::Ellipse {
                center,
                radii,
                rotation,
            },
            style,
        )
    }

    pub fn circle(center: Point, radius: f64, rotation: f64, style: &Style) -> Self {
        Self::new(
            SceneNodeKind::Circle {
                center,
                radius,
                rotation,
            },
            style,
        )
    }

    pub fn line(p0: Point, p1: Point, style: &Style) -> Self {
        Self::new(SceneNodeKind::Line { p0, p1 }, style)
    }

    pub fn rect(p0: Point, size: Size, style: &Style) -> Self {
        Self::new(SceneNodeKind::Rect { p0, size }, style)
    }

    pub fn round_rect(p0: Point, size: Size, radii: RoundedRectRadii, style: &Style) -> Self {
        Self::new(SceneNodeKind::RoundedRect { p0, size, radii }, style)
    }

    pub fn triangle(a: Point, b: Point, c: Point, style: &Style) -> Self {
        Self::new(SceneNodeKind::Triangle { a, b, c }, style)
    }

    pub fn quad_bez(a: Point, b: Point, c: Point, style: &Style) -> Self {
        Self::new(SceneNodeKind::QuadBez { a, b, c }, style)
    }

    pub fn cubic_bez(a: Point, b: Point, c: Point, d: Point, style: &Style) -> Self {
        Self::new(SceneNodeKind::CubicBez { a, b, c, d }, style)
    }

    pub fn bez_path(elements: Vec<PathEl>, style: &Style) -> Self {
        Self::new(SceneNodeKind::BezPath { elements }, style)
    }

    pub fn point(pos: Point, style: &Style) -> Self {
        Self::new(SceneNodeKind::Point { pos }, style)
    }

    pub fn arc(
        center: Point,
        radii: Vec2,
        start_angle: f64,
        sweep_angle: f64,
        rotation: f64,
        style: &Style,
    ) -> Self {
        Self::new(
            SceneNodeKind::Arc {
                center,
                radii,
                start_angle,
                sweep_angle,
                rotation,
            },
            style,
        )
    }
    pub fn point_light(
        center: Point,
        radius: f64,
        rotation: f64,
        color: Vec<(f32, SimpleColor)>,
    ) -> Self {
        Self {
            drawable: Some(SceneNodeKind::PointLight {
                center,
                radius,
                rotation,
                color,
            }),
            apply_parent_style: false,
            style: Style::default(),
            children: Default::default(),
        }
    }

    pub fn light_mask(screen_size: Size, lights: &Vec<(Point, f64)>, darkness_alpha: u8) -> Self {
        Self {
            drawable: Some(SceneNodeKind::LightMask {
                screen_size,
                lights: lights.clone(),
                darkness_alpha,
            }),
            apply_parent_style: false,
            children: Default::default(),
            style: Style::default(),
        }
    }
}

impl SceneNode {
    pub fn add_child(&mut self, other: &Self) {
        self.children.push(other.clone());
    }
    pub fn set_style(&mut self, style: &Style) {
        self.style = style.clone();
    }
}
