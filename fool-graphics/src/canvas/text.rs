use super::{Drawable, FontManager};
use super::{Style, TextFont};
use crate::graph_vec2;
use kurbo::Vec2;
use serde::{Deserialize, Serialize};
use skrifa::MetadataProvider;
use vello::kurbo::{Affine, Rect};
use vello::peniko::{BrushRef, StyleRef};
use vello::{Glyph, Scene};

pub type FontName = String;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct TextDrawable {
    pub position: Vec2,
    pub text: String,
    pub variations: Vec<(&'static str, f32)>,
    pub style: Style,
}

impl TextDrawable {
    pub fn draw_text(&self, scene: &mut Scene, style: &super::Style, font_mgr: FontManager) {
        let final_transform =
            style.translation * self.style.translation * Affine::translate(self.position);
        let font = font_mgr.get_font(&self.style.font.clone().unwrap_or_default());
        let _rect = Self::draw_glyphs(
            scene,
            &font,
            self.style.font_size.unwrap_or(16.0),
            &self.variations,
            &self.style.fill.clone().unwrap_or_default(),
            final_transform,
            None,
            self.style.fill_rule,
            &self.text,
            self.style.hint.unwrap_or_default(),
            self.style.align.clone().unwrap_or_default(),
            self.style.line_spacing.unwrap_or(1.0),
        );
        #[cfg(feature = "debug")]
        {
            scene.stroke(
                &Default::default(),
                final_transform * Affine::translate(graph_vec2!(_rect.x0, _rect.y0)),
                &Brush::Solid(Color::from_rgba8(255, 0, 0, 255)),
                Some(final_transform),
                &_rect,
            );
        }
    }

    fn draw_glyphs<'a>(
        scene: &mut Scene,
        font: &TextFont,
        size: f32,
        variations: &[(&str, f32)],
        brush: impl Into<BrushRef<'a>>,
        transform: Affine,
        glyph_transform: Option<Affine>,
        style: impl Into<StyleRef<'a>>,
        text: &str,
        hint: bool,
        align: TextAlign,
        line_spacing: f32,
    ) -> Rect {
        let font_ref = font.as_ref();
        let brush = brush.into();
        let style = style.into();
        let axes = font_ref.axes();
        let font_size = skrifa::instance::Size::new(size);
        let var_loc = axes.location(variations.iter().copied());
        let charmap = font_ref.charmap();
        let metrics = font_ref.metrics(font_size, &var_loc);
        let line_height = (metrics.ascent - metrics.descent + metrics.leading) * line_spacing;
        let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);

        let lines: Vec<&str> = text.lines().collect();
        let mut pen_y = metrics.ascent;
        let mut max_width = 0.0f32;
        let mut num_lines = 0;
        let mut x = 0.0f32;
        for line in lines {
            let mut line_width = 0.0;
            for ch in line.chars().filter(|ch| !ch.is_control()) {
                let gid = charmap.map(ch).unwrap_or_default();
                line_width += glyph_metrics.advance_width(gid).unwrap_or_default();
            }
            num_lines += 1;
            max_width = max_width.max(line_width);
            let offset_x = match align {
                TextAlign::Left => 0.0,
                TextAlign::Center => -line_width / 2.0,
                TextAlign::Right => -line_width,
            };
            x = x.min(offset_x);
            let mut pen_x = 0.0;
            let glyph_iter = line.chars().filter(|ch| !ch.is_control()).filter_map(|ch| {
                let gid = charmap.map(ch).unwrap_or_default();
                let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                let x = pen_x + offset_x;
                let y = pen_y;
                pen_x += advance;
                Some(Glyph {
                    id: gid.to_u32(),
                    x,
                    y,
                })
            });
            scene
                .draw_glyphs(&font.font)
                .font_size(size)
                .transform(transform)
                .glyph_transform(glyph_transform)
                .brush(brush.clone())
                .hint(hint)
                .draw(style, glyph_iter);
            pen_y += line_height;
        }
        let height = num_lines as f32 * line_height;
        Rect::new(x as f64, 0.0, max_width as f64, height as f64)
    }
}

impl TextDrawable {
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_variations(mut self, variations: Vec<(&'static str, f32)>) -> Self {
        self.variations = variations;
        self
    }
}
impl Default for TextDrawable {
    fn default() -> Self {
        Self {
            position: graph_vec2!(0.0, 0.0),
            text: "Hello\nWorld!".to_string(),
            variations: vec![],
            style: Default::default(),
        }
    }
}

impl Drawable for TextDrawable {
    fn draw(&self, scene: &mut Scene, style: &super::Style, font_mgr: FontManager) {
        self.draw_text(scene, style, font_mgr);
    }
}
