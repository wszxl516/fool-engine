use super::utils::texture_from_image;
use egui::epaint::TextureHandle;
use egui::Context;
use fool_graphics::canvas::{Blob, Image, ImageFormat};
use fool_resource::{Fallback, Resource, SharedData};
use image::DynamicImage;
use image::GenericImageView;
use std::fmt::Debug;
use std::io::Read;
use std::sync::Arc;
use winit::window::Icon;
#[derive(Debug, Clone)]
pub struct FSFallBack {
    pub asset_path: std::path::PathBuf,
}

impl Fallback for FSFallBack {
    type K = String;
    type V = SharedData;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let full_path = self.asset_path.join(key);
        let mut fd = std::fs::File::open(full_path)?;
        let mut buffer = Vec::new();
        fd.read_to_end(&mut buffer)?;
        Ok(SharedData::from(buffer))
    }
}

#[derive(Clone)]
pub struct EguiTextureFallBack {
    pub raw_image: Resource<String, Arc<DynamicImage>>,
    pub ctx: Context,
}
impl Debug for EguiTextureFallBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EguiTextureFallBack")
    }
}
impl Fallback for EguiTextureFallBack {
    type K = String;
    type V = TextureHandle;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let img = self.raw_image.get(key)?;
        let texture = texture_from_image(&key, &img, &self.ctx)?;
        Ok(texture)
    }
}

#[derive(Clone)]
pub struct WindowIconFallBack {
    pub raw_image: Resource<String, Arc<DynamicImage>>,
}
impl Debug for WindowIconFallBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowIconFallBack")
    }
}
impl Fallback for WindowIconFallBack {
    type K = String;
    type V = Arc<Icon>;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let img = self.raw_image.get(key)?;
        let width = img.width();
        let height = img.height();
        let rgba = img
            .as_rgba8()
            .ok_or(anyhow::anyhow!("convert {} to rgba8 failed!", key))?;
        let icon = Icon::from_rgba(rgba.clone().into_vec(), width, height)?;
        Ok(Arc::new(icon))
    }
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct WindowCursorFallBack {
    pub raw_image: Resource<String, Arc<DynamicImage>>,
}
impl Debug for WindowCursorFallBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowCursorFallBack")
    }
}
impl Fallback for WindowCursorFallBack {
    type K = String;
    type V = Arc<Icon>;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let img = self.raw_image.get(key)?;
        let width = img.width();
        let height = img.height();
        let rgba = img
            .as_rgba8()
            .ok_or(anyhow::anyhow!("convert {} to rgba8 failed!", key))?;
        let icon = Icon::from_rgba(rgba.clone().into_vec(), width, height)?;
        Ok(Arc::new(icon))
    }
}
#[derive(Clone)]
pub struct RawImageFallBack {
    pub raw_data: Resource<String, SharedData>,
}
impl Debug for RawImageFallBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawImageFallBack")
    }
}
impl Fallback for RawImageFallBack {
    type K = String;
    type V = Arc<DynamicImage>;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let img = self.raw_data.get(key)?;
        let img = img.to_image()?;
        Ok(Arc::new(img))
    }
}

#[derive(Clone)]
pub struct VelloImageFallBack {
    pub raw_image: Resource<String, Arc<DynamicImage>>,
}
impl Debug for VelloImageFallBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EguiTextureFallBack")
    }
}
impl Fallback for VelloImageFallBack {
    type K = String;
    type V = Arc<Image>;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let img = self.raw_image.get(key)?;
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        let image = Image {
            width,
            height,
            data: Blob::from(rgba.into_vec()),
            format: ImageFormat::Rgba8,
            x_extend: Default::default(),
            y_extend: Default::default(),
            quality: Default::default(),
            alpha: 1.0,
        };
        Ok(Arc::new(image))
    }
}
