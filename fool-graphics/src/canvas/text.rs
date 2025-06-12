use super::ImageManager;
use super::{Drawable, FontManager, Style, VelloFont};
use crate::graph_pt2;
use kurbo::{Point, Size};
use serde::{Deserialize, Serialize};
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
    pub position: Point,
    pub text: String,
    pub style: Style,
}

impl TextDrawable {
    pub fn draw_text(
        &self,
        scene: &mut Scene,
        style: &super::Style,
        font_mgr: FontManager,
        img_res: ImageManager,
    ) -> anyhow::Result<()> {
        let final_transform =
            self.style.translation * style.translation * Affine::translate(self.position.to_vec2());
        let font = font_mgr.get(&self.style.font.clone().unwrap_or_default());
        let brush = match &self.style.fill {
            Some(b) => b.build(img_res)?,
            None => Default::default(),
        };
        let _rect = if self.style.vertical.unwrap_or(false) {
            Self::draw_glyphs_vertical(
                scene,
                &font,
                self.style.font_size.unwrap_or(16.0),
                &brush,
                final_transform,
                None,
                self.style.fill_rule,
                &self.text,
                self.style.hint.unwrap_or_default(),
                self.style.align.clone().unwrap_or_default(),
                self.style.line_spacing.unwrap_or(1.0),
            )
        } else {
            Self::draw_glyphs_horizontal(
                scene,
                &font,
                self.style.font_size.unwrap_or(16.0),
                &brush,
                final_transform,
                None,
                self.style.fill_rule,
                &self.text,
                self.style.hint.unwrap_or_default(),
                self.style.align.clone().unwrap_or_default(),
                self.style.line_spacing.unwrap_or(1.0),
            )
        };

        #[cfg(feature = "graph_debug")]
        {
            use vello::peniko::{Brush, Color};
            scene.stroke(
                &Default::default(),
                final_transform,
                &Brush::Solid(Color::from_rgba8(255, 0, 0, 255)),
                // Some(final_transform),
                None,
                &_rect,
            );
        }
        Ok(())
    }

    pub fn draw_glyphs_vertical<'a>(
        scene: &mut Scene,
        font: &VelloFont,
        size: f32,
        brush: impl Into<BrushRef<'a>>,
        transform: Affine,
        glyph_transform: Option<Affine>,
        style: impl Into<StyleRef<'a>>,
        text: &String,
        hint: bool,
        align: TextAlign,
        line_spacing: f32,
    ) -> Rect {
        let glyphs = font.glyph(text, size);
        let brush = brush.into();
        let style = style.into();
        let mut glyphs_index = 0usize;

        let (line_height, baseline_offset) = glyphs
            .get(0)
            .map(|gly| {
                let h =
                    gly.metrics.ascent - gly.metrics.descent + gly.metrics.leading + line_spacing;
                (h, gly.metrics.ascent)
            })
            .unwrap_or((1.0, 0.0));
        let lines: Vec<&str> = text.lines().collect();
        let num_cols = lines.len();
        let col_width = size;
        let total_width = col_width * num_cols as f32;
        let start_x = match align {
            TextAlign::Left => 0.0,
            TextAlign::Center => -total_width / 2.0,
            TextAlign::Right => -total_width,
        };

        let mut pen_x = start_x;
        let mut max_y = 0.0f32;
        let mut min_x = pen_x;
        let mut max_x = pen_x;

        let mut glyph_iter = Vec::new();

        for line in lines {
            let mut pen_y = 0.0;
            for ch in line.chars() {
                if !ch.is_control() {
                    if let Some(gly) = glyphs.get(glyphs_index) {
                        let advance = line_height;
                        let gid = gly.gid;

                        let x = pen_x;
                        let y = pen_y + baseline_offset;

                        glyph_iter.push(Glyph {
                            id: gid.to_u32(),
                            x,
                            y,
                        });

                        pen_y += advance;

                        max_y = max_y.max(y);
                        min_x = min_x.min(x);
                        max_x = max_x.max(x);
                    }
                }
                glyphs_index += 1;
            }
            glyphs_index += 1;
            pen_x += col_width;
        }

        scene
            .draw_glyphs(&font.font)
            .font_size(size)
            .transform(transform)
            .glyph_transform(glyph_transform)
            .brush(brush)
            .hint(hint)
            .draw(style, glyph_iter.into_iter());

        let width = (max_x - min_x + size) as f64;
        let height = max_y as f64;
        Rect::from_origin_size(
            graph_pt2!(min_x as _, 0.0),
            Size::new(width as _, height as _),
        )
    }

    fn draw_glyphs_horizontal<'a>(
        scene: &mut Scene,
        font: &VelloFont,
        size: f32,
        brush: impl Into<BrushRef<'a>>,
        transform: Affine,
        glyph_transform: Option<Affine>,
        style: impl Into<StyleRef<'a>>,
        text: &String,
        hint: bool,
        align: TextAlign,
        line_spacing: f32,
    ) -> Rect {
        let glyphs = font.glyph(text, size);
        let mut glyphs_index = 0usize;
        let brush = brush.into();
        let style = style.into();
        let (line_height, mut pen_y) = glyphs
            .get(0)
            .and_then(|gly| {
                Some((
                    (gly.metrics.ascent - gly.metrics.descent + gly.metrics.leading + line_spacing),
                    gly.metrics.ascent,
                ))
            })
            .unwrap_or((1.0, 0.0));
        let lines: Vec<&str> = text.lines().collect();
        let mut max_width = 0.0f32;
        let mut num_lines = 0;
        let mut min_x = 0.0f32;
        for line in lines {
            let mut line_width = 0.0;
            num_lines += 1;
            let offset_x = match align {
                TextAlign::Left => 0.0,
                TextAlign::Center => -line_width / 2.0,
                TextAlign::Right => -line_width,
            };
            min_x = min_x.min(offset_x);
            let mut pen_x = 0.0;
            let mut glyph_iter = Vec::new();
            for ch in line.chars() {
                if !ch.is_control() {
                    let gly = &glyphs[glyphs_index];
                    let gid = gly.gid;
                    let advance = gly.advance_width.unwrap_or_default();
                    let x = pen_x + offset_x;
                    let y = pen_y;
                    pen_x += advance;
                    line_width += advance;
                    glyph_iter.push(Glyph {
                        id: gid.to_u32(),
                        x,
                        y,
                    })
                }

                glyphs_index += 1;
            }
            max_width = max_width.max(line_width);
            scene
                .draw_glyphs(&font.font)
                .font_size(size)
                .transform(transform)
                .glyph_transform(glyph_transform)
                .brush(brush.clone())
                .hint(hint)
                .draw(style, glyph_iter.into_iter());
            pen_y += line_height;
            // \n
            glyphs_index += 1;
        }
        let height = num_lines as f32 * line_height;
        Rect::from_origin_size(graph_pt2!(0.0, 0.0), Size::new(max_width as _, height as _))
    }
}

impl TextDrawable {
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}
impl Default for TextDrawable {
    fn default() -> Self {
        Self {
            position: Point { x: 0.0, y: 0.0 },
            text: "Hello\nWorld!".to_string(),
            style: Default::default(),
        }
    }
}
impl Drawable for TextDrawable {
    fn draw(
        &self,
        scene: &mut Scene,
        style: &super::Style,
        font_mgr: FontManager,
        img_res: ImageManager,
    ) -> anyhow::Result<()> {
        self.draw_text(scene, style, font_mgr, img_res)?;
        Ok(())
    }
}
