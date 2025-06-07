#![allow(unused_imports)]
mod types;
use fool_resource::{Fallback, Resource, SharedData};
pub use types::{FontGlyph, FontGlyphId, VelloFont};
use vello::peniko::Blob;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct FontManager {
    cache: Resource<String, VelloFont>,
}
impl Default for FontManager {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}
impl FontManager {
    pub fn new(fall_back: impl Fallback<K = String, V = VelloFont> + 'static) -> Self {
        let this = Self::default();
        this.cache.set_fall_back(fall_back);
        this
    }
    pub fn default_font(&self) -> VelloFont {
        VelloFont::default()
    }
    pub fn get(&self, name: &str) -> VelloFont {
        self.cache.get(name).unwrap_or(self.default_font())
    }
    pub fn put(&self, name: &str, data: SharedData) {
        let font = VelloFont::new(Blob::from(data.to_vec()));
        self.cache.load(name.to_string(), font);
    }
    pub fn exists(&self, name: &str) -> bool {
        self.cache.exists(name)
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct VelloFontFallback {
    resource: Resource<String, SharedData>,
}
impl std::fmt::Debug for VelloFontFallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VelloFontFallback")
    }
}
impl VelloFontFallback {
    pub fn from_resource(resource: Resource<String, SharedData>) -> Self {
        Self { resource }
    }
}
impl Fallback for VelloFontFallback {
    type K = String;
    type V = VelloFont;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let data = self.resource.get(key)?;
        Ok(VelloFont::new(Blob::from(data.to_vec())))
    }
}
