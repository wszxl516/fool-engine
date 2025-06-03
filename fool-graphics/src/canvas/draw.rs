use std::fmt::Debug;

use super::{FontManager, Style};
use vello::{Scene, kurbo::Shape};
pub trait Drawable: DrawableClone + Debug {
    fn draw(&self, scene: &mut Scene, style: &Style, font_mgr: FontManager);
}

impl<T: Shape + Sized + Debug + Clone + 'static> Drawable for T {
    fn draw(&self, scene: &mut Scene, style: &Style, _font_mgr: FontManager) {
        if !style.visible {
            return;
        }
        if let Some(brush) = &style.fill {
            let brush = brush.clone().multiply_alpha(style.opacity);
            scene.fill(style.fill_rule, style.translation, &brush, None, self);
        }
        if let Some(stoke) = &style.stoke {
            let brush = stoke.brush.clone().multiply_alpha(style.opacity);
            scene.stroke(
                &stoke.stroke,
                style.translation,
                &brush,
                Some(style.translation),
                self,
            );
        }
    }
}

pub trait DrawableClone {
    fn clone_box(&self) -> Box<dyn Drawable>;
}

impl<T> DrawableClone for T
where
    T: 'static + Drawable + Clone + Sized,
{
    fn clone_box(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Drawable> {
    fn clone(&self) -> Box<dyn Drawable> {
        self.clone_box()
    }
}
