#![allow(unused_imports)]
mod types;
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};
pub use types::{DEFAULT_FONT_NAME, FontGlyph, FontGlyphId, VelloFont};
use vello::peniko::Blob;

#[derive(Debug, Clone)]
pub struct FontManager {
    cache: Arc<RwLock<HashMap<String, VelloFont>>>,
}
impl Default for FontManager {
    fn default() -> Self {
        let mut cache: HashMap<String, VelloFont> = Default::default();
        cache.insert(DEFAULT_FONT_NAME.to_string(), VelloFont::default());
        Self {
            cache: Arc::new(RwLock::new(cache)),
        }
    }
}
impl FontManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn default_font(&self) -> VelloFont {
        self.cache
            .read()
            .get(DEFAULT_FONT_NAME)
            .cloned()
            .unwrap_or_default()
    }
    pub fn get(&self, name: &str) -> VelloFont {
        self.cache
            .read()
            .get(name)
            .cloned()
            .unwrap_or(self.default_font())
    }
    pub fn put(&self, name: &str, data: Arc<Vec<u8>>) {
        let font = VelloFont::new(Blob::new(data));
        self.cache.write().insert(name.to_string(), font);
    }
    pub fn exists(&self, name: &str) -> bool {
        self.cache.read().contains_key(name)
    }
}
