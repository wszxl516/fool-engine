use crate::{
    canvas::{SceneNode, SceneNodeKind},
    graph_pt2,
};
use image::{DynamicImage, GenericImageView};
use std::sync::Arc;
use vello::peniko::{Blob, Image, ImageFormat};
#[derive(Debug, Clone)]
pub struct Frame {
    pub img: Arc<Image>,
}
impl Frame {
    pub fn from_image(img: &DynamicImage) -> Self {
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        Frame {
            img: Arc::new(Image {
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
    pub fn to_node(&self, x: f64, y: f64) -> SceneNode {
        let node = SceneNodeKind::SpriteImage {
            position: graph_pt2!(x, y),
            image: self.img.as_ref().clone(),
        };
        SceneNode {
            style: Default::default(),
            drawable: Some(node),
            apply_parent_style: true,
            children: Default::default(),
        }
    }
}
