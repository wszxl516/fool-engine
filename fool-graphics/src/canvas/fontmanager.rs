use parking_lot::RwLock;
use skrifa::raw::{FileRef, FontRef};
use std::{collections::HashMap, sync::Arc};
use vello::peniko::{Blob, Font};
const DEFAULT_FONT: &[u8] = include_bytes!("../../Roboto_SemiCondensed-Bold.ttf");
const DEFAULT_FONT_NAME: &str = "Roboto_SemiCondensed-Bold.ttf";
fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
    let file_ref = FileRef::new(font.data.as_ref()).ok()?;
    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}

#[derive(Debug, Clone)]
pub struct TextFont {
    pub font: Font,
}

impl TextFont {
    pub fn new(data: Blob<u8>) -> Self {
        Self {
            font: Font::new(Blob::new(Arc::new(data)), 0),
        }
    }
    pub fn as_ref(&self) -> FontRef<'_> {
        to_font_ref(&self.font).unwrap()
    }
}

impl Default for TextFont {
    fn default() -> Self {
        Self::new(DEFAULT_FONT.to_vec().into())
    }
}

#[derive(Debug, Clone)]
pub struct FontManager {
    cache: Arc<RwLock<HashMap<String, TextFont>>>,
}
impl Default for FontManager {
    fn default() -> Self {
        let mut cache: HashMap<String, TextFont> = Default::default();
        cache.insert(DEFAULT_FONT_NAME.to_string(), TextFont::default());
        Self {
            cache: Arc::new(RwLock::new(cache)),
        }
    }
}
impl FontManager {
    pub fn empty() -> Self {
        Self::default()
    }
    pub fn get_font(&self, name: &str) -> TextFont {
        self.cache
            .read()
            .get(name)
            .cloned()
            .or_else(|| self.cache.read().get(DEFAULT_FONT_NAME).cloned())
            .unwrap()
    }
    pub fn put_font(&self, name: &str, data: impl Into<Blob<u8>>) {
        let font = TextFont::new(data.into());
        self.cache.write().insert(name.to_string(), font);
    }
}
