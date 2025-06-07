use ordered_float::OrderedFloat;
use parking_lot::RwLock;
use skrifa::{
    GlyphId, MetadataProvider,
    instance::Size,
    metrics::{BoundingBox, Metrics},
    raw::{FileRef, FontRef},
};
use std::{collections::HashMap, sync::Arc};
use vello::peniko::{Blob, Font};
fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
    let file_ref = FileRef::new(font.data.as_ref()).ok()?;
    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontGlyphId {
    pub char: char,
    pub font_size: OrderedFloat<f32>,
}
#[derive(Clone, Debug)]
pub struct FontGlyph {
    pub gid: GlyphId,
    pub advance_width: Option<f32>,
    pub left_side_bearing: Option<f32>,
    pub bounds: Option<BoundingBox>,
    pub metrics: Metrics,
}
impl Default for FontGlyph {
    fn default() -> Self {
        Self {
            gid: GlyphId::NOTDEF,
            advance_width: None,
            left_side_bearing: None,
            bounds: None,
            metrics: Default::default(),
        }
    }
}
impl FontGlyph {
    pub fn from_font(font: &Font, ch: char, size: f32) -> Option<Self> {
        to_font_ref(font).and_then(|f| {
            let font_size = Size::new(size);
            let axes = f.axes();
            let variations: &[(&str, f32)] = &[];
            let var_loc = axes.location(variations);
            let charmap = f.charmap();
            let gid = charmap.map(ch).unwrap_or_default();
            let glyph_metrics = f.glyph_metrics(font_size, &var_loc);
            let metrics = f.metrics(font_size, &var_loc);
            let advance_width = glyph_metrics.advance_width(gid);
            let bounds = glyph_metrics.bounds(gid);
            let left_side_bearing = glyph_metrics.left_side_bearing(gid);
            Some(FontGlyph {
                gid,
                advance_width,
                left_side_bearing,
                bounds,
                metrics,
            })
        })
    }
    #[inline]
    pub fn exists(&self) -> bool {
        self.gid != GlyphId::NOTDEF
    }
}
const DEFAULT_FALLBACK_FONT: &[u8] = include_bytes!("../../../Roboto_SemiCondensed-Bold.ttf");
#[derive(Clone, Debug)]
pub struct VelloFont {
    pub font: Font,
    glyph: Arc<RwLock<HashMap<FontGlyphId, Arc<FontGlyph>>>>,
}
impl Default for VelloFont {
    fn default() -> Self {
        Self {
            font: Font::new(Blob::new(Arc::new(DEFAULT_FALLBACK_FONT)), 0),
            glyph: Default::default(),
        }
    }
}
impl VelloFont {
    pub fn new(data: Blob<u8>) -> Self {
        Self {
            font: Font::new(Blob::new(Arc::new(data)), 0),
            glyph: Default::default(),
        }
    }
    pub fn glyph(&self, text: &String, size: f32) -> Vec<Arc<FontGlyph>> {
        let mut lock = self.glyph.write();
        let mut glyphs = Vec::new();
        for ch in text.as_str().chars() {
            let id = FontGlyphId {
                char: ch,
                font_size: OrderedFloat::from(size),
            };
            let g = lock
                .entry(id)
                .or_insert_with(|| {
                    let glyph =
                        FontGlyph::from_font(&self.font, ch, size).unwrap_or(FontGlyph::default());
                    Arc::new(glyph)
                })
                .clone();
            glyphs.push(g);
        }
        glyphs
    }
}
