use super::ImageManager;
use super::text::{FontName, TextAlign};
use peniko::Brush;
use serde::{Deserialize, Serialize};
pub use vello::{
    kurbo::{Affine, Stroke},
    peniko::{
        Color, ColorStop, ColorStops, Extend, Fill, Gradient, GradientKind,
        color::{ColorSpaceTag, HueDirection},
    },
};
#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct StokeStyle {
    #[serde(default)]
    pub stroke: Stroke,
    pub brush: CustomBrush,
}
const fn default_fill() -> Fill {
    Fill::NonZero
}
const fn default_opacity() -> f32 {
    1.0
}
const fn default_visible() -> bool {
    true
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SimpleColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Default for SimpleColor {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum CustomBrush {
    Color(SimpleColor),
    Gradient(CustomGradient),
    Image(String),
}
impl Default for CustomBrush {
    fn default() -> Self {
        Self::Color(SimpleColor::default())
    }
}

impl CustomBrush {
    pub fn build(&self, img_res: ImageManager) -> anyhow::Result<Brush> {
        match self {
            Self::Color(color) => Ok(Brush::Solid(
                Color::from_rgba8(color.r, color.g, color.b, color.a).into(),
            )),
            Self::Gradient(gradient) => Ok(Brush::Gradient(gradient.to_vello())),
            Self::Image(img) => Ok(Brush::Image(img_res.get(img)?.as_ref().clone())),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub enum CustomGradientKind {
    #[default]
    Linear,
    Radial,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub enum CustomExtend {
    #[default]
    Pad,
    Repeat,
    Reflect,
}
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CustomGradient {
    pub kind: CustomGradientKind,
    pub extend: CustomExtend,
    pub colors: Vec<(f32, SimpleColor)>,
}
impl CustomGradient {
    pub fn to_vello(&self) -> peniko::Gradient {
        let kind = match self.kind {
            CustomGradientKind::Linear => GradientKind::Linear {
                start: (0.0, 0.0).into(),
                end: (1.0, 0.0).into(),
            },
            CustomGradientKind::Radial => GradientKind::Radial {
                start_center: (0.0, 0.0).into(),
                end_center: (0.0, 0.0).into(),
                start_radius: 0.0,
                end_radius: 1.0,
            },
        };

        let extend = match self.extend {
            CustomExtend::Pad => Extend::Pad,
            CustomExtend::Repeat => Extend::Repeat,
            CustomExtend::Reflect => Extend::Reflect,
        };
        let colors = self
            .colors
            .iter()
            .map(|(pos, color)| ColorStop {
                offset: *pos,
                color: Color::from_rgba8(color.r, color.g, color.b, color.a).into(),
            })
            .collect();
        let stops = ColorStops(colors);
        Gradient {
            kind,
            extend,
            interpolation_cs: ColorSpaceTag::Srgb,
            hue_direction: HueDirection::Shorter,
            stops,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Style {
    #[serde(default)]
    pub translation: Affine,
    #[serde(default)]
    pub fill: Option<CustomBrush>,
    #[serde(default = "default_fill")]
    pub fill_rule: Fill,
    #[serde(default)]
    pub stoke: Option<StokeStyle>,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default)]
    pub z_index: i32,
    #[serde(default)]
    pub tag: Option<String>,
    // for text
    #[serde(default)]
    pub font: Option<FontName>,
    #[serde(default)]
    pub font_size: Option<f32>,
    #[serde(default)]
    pub hint: Option<bool>,
    #[serde(default)]
    pub align: Option<TextAlign>,
    #[serde(default)]
    pub line_spacing: Option<f32>,
    #[serde(default)]
    pub vertical: Option<bool>,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            translation: Affine::IDENTITY,
            fill: Some(CustomBrush::default()),
            fill_rule: Fill::NonZero,
            stoke: Default::default(),
            opacity: 1.0,
            visible: true,
            z_index: 0,
            tag: None,
            font: Default::default(),
            font_size: None,
            hint: None,
            align: None,
            line_spacing: None,
            vertical: Some(false),
        }
    }
}

impl Style {
    fn mul_ops(&self, child: &Style) -> Style {
        let translation = self.translation * child.translation;
        let fill = child.fill.clone();
        let fill_rule = child.fill_rule;
        let stoke = child.stoke.clone();
        let opacity = self.opacity * child.opacity;
        let visible = self.visible && child.visible;
        let z_index = self.z_index + child.z_index;
        let tag = child.tag.clone().or_else(|| self.tag.clone());
        let font = child.font.clone().or_else(|| self.font.clone());
        let font_size = child.font_size.or_else(|| self.font_size.clone());
        let hint = child.hint.or_else(|| self.hint.clone());
        let align = child.align.clone().or_else(|| self.align.clone());
        let line_spacing = child.line_spacing.or_else(|| self.line_spacing.clone());
        let vertical = child.vertical.or_else(|| self.vertical);
        Style {
            font,
            font_size,
            hint,
            align,
            line_spacing,
            translation,
            fill,
            fill_rule,
            stoke,
            opacity,
            visible,
            z_index,
            tag,
            vertical,
        }
    }
}

impl std::ops::Mul for Style {
    type Output = Style;
    fn mul(self, rhs: Style) -> Style {
        self.mul_ops(&rhs)
    }
}
impl<'a> std::ops::Mul<&'a Style> for &'a Style {
    type Output = Style;
    fn mul(self, rhs: &'a Style) -> Self::Output {
        self.mul_ops(rhs)
    }
}
impl Style {
    pub fn with_translation(mut self, translation: Affine) -> Self {
        self.translation = translation;
        self
    }

    pub fn with_fill(mut self, fill: Option<CustomBrush>) -> Self {
        self.fill = fill;
        self
    }

    pub fn with_fill_rule(mut self, fill_rule: Fill) -> Self {
        self.fill_rule = fill_rule;
        self
    }

    pub fn with_stoke(mut self, stroke: Option<StokeStyle>) -> Self {
        self.stoke = stroke;
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn with_tag<T: Into<String>>(mut self, tag: T) -> Self {
        self.tag = Some(tag.into());
        self
    }

    pub fn with_font_size(mut self, size: Option<f32>) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_font(mut self, font: Option<FontName>) -> Self {
        self.font = font;
        self
    }

    pub fn with_hint(mut self, hint: Option<bool>) -> Self {
        self.hint = hint;
        self
    }

    pub fn with_align(mut self, align: Option<TextAlign>) -> Self {
        self.align = align;
        self
    }

    pub fn with_line_spacing(mut self, spacing: Option<f32>) -> Self {
        self.line_spacing = spacing;
        self
    }
    pub fn with_vertical(mut self, vertical: Option<bool>) -> Self {
        self.vertical = vertical;
        self
    }
}
