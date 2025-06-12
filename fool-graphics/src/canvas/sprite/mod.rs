use image::{DynamicImage, GenericImageView};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    sync::Arc,
};
mod animation;
mod frame;
mod scheduler;
pub use animation::Animation;
pub use frame::Frame;
pub use scheduler::Scheduler;
pub trait FrameId: Hash + Clone + Eq + PartialEq + Display + Debug + Send + Sync {}
impl<T: Hash + Clone + Eq + PartialEq + Display + Debug + Send + Sync> FrameId for T {}
pub struct Sprite<Id: FrameId> {
    frames: HashMap<Id, Frame>,
    animation: HashMap<String, Animation>,
}

impl<Id: FrameId> Sprite<Id> {
    pub fn from_image(
        image: Arc<DynamicImage>,
        tile_width: u32,
        tile_height: u32,
        frames_ids: impl Iterator<Item = Id>,
    ) -> Self {
        let (img_width, img_height) = image.dimensions();
        let tiles_x = img_width / tile_width;
        let tiles_y = img_height / tile_height;
        let mut frames_ids = frames_ids;
        let mut frames = HashMap::new();
        for y in 0..tiles_y {
            for x in 0..tiles_x {
                let id = frames_ids.next();
                if let Some(id) = id {
                    let img =
                        image.crop_imm(x * tile_width, y * tile_height, tile_width, tile_height);
                    let sprite = Frame::from_image(&img);
                    frames.insert(id, sprite);
                }
            }
        }
        Self {
            frames,
            animation: Default::default(),
        }
    }
    pub fn create_animation(
        &mut self,
        name: impl Into<String>,
        frames_num: impl Iterator<Item = Id>,
        fps: u32,
    ) -> anyhow::Result<Animation> {
        let mut frames = Vec::new();
        for n in frames_num {
            if let Some(frame) = self.frames.get(&n) {
                frames.push(frame.clone());
            } else {
                return Err(anyhow::anyhow!("Id {} of Frame not found!", n));
            }
        }
        let animation = Animation::new(frames, fps);
        self.animation.insert(name.into(), animation.clone());
        Ok(animation)
    }
    pub fn get_animation(&self, name: impl Into<String>) -> anyhow::Result<Animation> {
        let name = name.into();
        self.animation
            .get(&name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Id {} of Animation not found!", name))
    }
    pub fn list_animation(&self) -> Vec<String> {
        self.animation.keys().cloned().collect()
    }
}
