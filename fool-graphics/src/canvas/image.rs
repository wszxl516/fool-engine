use super::ImageManager;
use kurbo::{Affine, Point, Rect, Size};
use peniko::Image;
use serde::{Deserialize, Serialize};

use crate::{
    canvas::{Drawable, Style},
    graph_vec2,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VelloImage {
    Path(String),
    Image(Image),
}
impl VelloImage {
    pub fn draw(
        &self,
        x: f64,
        y: f64,
        scene: &mut vello::Scene,
        style: &Style,
        resource: ImageManager,
    ) -> anyhow::Result<()> {
        match self {
            Self::Image(img) => {
                let (width, height) = (img.width as f64, img.height as f64);
                let rect = Rect::from_center_size(Point::new(x, y), Size::new(width, height));
                let tex_to_rect = Affine::translate(graph_vec2!(x - width / 2.0, y - height / 2.0));
                scene.fill(
                    peniko::Fill::NonZero,
                    Affine::IDENTITY * style.translation,
                    img,
                    Some(tex_to_rect),
                    &rect,
                );
                #[cfg(feature = "graph_debug")]
                {
                    let bg = peniko::Color::from_rgba8(255, 0, 0, 255);
                    scene.stroke(
                        &Default::default(),
                        Affine::IDENTITY * style.translation,
                        &bg,
                        Some(tex_to_rect),
                        &rect,
                    );
                }
            }
            Self::Path(path) => {
                let img = resource.get(path)?;
                let (width, height) = (img.width as f64, img.height as f64);
                let rect = Rect::from_center_size(Point::new(x, y), Size::new(width, height));
                let tex_to_rect = Affine::translate(graph_vec2!(x - width / 2.0, y - height / 2.0));
                scene.fill(
                    peniko::Fill::NonZero,
                    Affine::IDENTITY * style.translation,
                    img.as_ref(),
                    Some(tex_to_rect),
                    &rect,
                );
                #[cfg(feature = "graph_debug")]
                {
                    let bg = peniko::Color::from_rgba8(255, 0, 0, 255);
                    scene.stroke(
                        &Default::default(),
                        Affine::IDENTITY * style.translation,
                        &bg,
                        Some(tex_to_rect),
                        &rect,
                    );
                }
            }
        };

        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct ImageDrawable {
    pub position: Point,
    pub image: VelloImage,
}
impl Drawable for ImageDrawable {
    fn draw(
        &self,
        scene: &mut vello::Scene,
        style: &super::Style,
        _font_mgr: super::FontManager,
        img_mgr: ImageManager,
    ) -> anyhow::Result<()> {
        self.image
            .draw(self.position.x, self.position.y, scene, style, img_mgr)
    }
}
