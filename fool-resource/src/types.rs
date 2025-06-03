pub use image::DynamicImage;
use std::{ops::Deref, sync::Arc};

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct SharedData(Arc<[u8]>);
impl Deref for SharedData {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<[u8]> for SharedData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&[u8]> for SharedData {
    fn from(value: &[u8]) -> Self {
        SharedData(Arc::from(value.to_vec().into_boxed_slice()))
    }
}
impl From<&str> for SharedData {
    fn from(value: &str) -> Self {
        SharedData(Arc::from(value.as_bytes()))
    }
}
impl From<Vec<u8>> for SharedData {
    fn from(data: Vec<u8>) -> Self {
        SharedData::from_vec(data)
    }
}
impl SharedData {
    pub fn from_vec(data: Vec<u8>) -> Self {
        Self(Arc::from(data.into_boxed_slice()))
    }
    pub fn from_static(data: &'static [u8]) -> Self {
        Self(Arc::from(data))
    }
    pub fn to_image(&self) -> anyhow::Result<DynamicImage> {
        Ok(image::load_from_memory(&self)?)
    }
    pub fn to_string(&self) -> anyhow::Result<String> {
        Ok(String::from_utf8(self.0.to_vec())?)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
