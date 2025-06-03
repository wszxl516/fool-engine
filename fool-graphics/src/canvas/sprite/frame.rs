use image::{DynamicImage, GenericImageView};
use std::sync::Arc;
use vello::peniko::{Blob, Image as VelloImage, ImageFormat};

use crate::canvas::{SceneNode, Style};
#[derive(Debug, Clone)]
pub struct Frame {
    pub img: Arc<VelloImage>,
}
impl Frame {
    pub fn from_image(img: &DynamicImage) -> Self {
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        Frame {
            img: Arc::new(VelloImage {
                width,
                height,
                data: Blob::from(rgba.into_vec()),
                format: ImageFormat::Rgba8,
                x_extend: Default::default(),
                y_extend: Default::default(),
                quality: Default::default(),
                alpha: 1.0,
            }),
        }
    }
    pub fn from_image_with_rect(
        img: &DynamicImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Self {
        let img = img.crop_imm(x, y, width, height);
        Self::from_image(&img)
    }
    pub fn to_node(&self, style: &Style) -> SceneNode {
        SceneNode::image(self.img.as_ref().clone(), style)
    }
}
