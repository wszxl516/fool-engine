use super::Frame;
use super::Scheduler;
use crate::canvas::SceneNode;
use std::sync::Arc;
#[derive(Debug, Clone, Default)]
pub struct Animation {
    frames: Arc<Vec<Frame>>,
    on_pause: Option<Frame>,
    running: bool,
    pub scheduler: Scheduler,
    count: usize,
    current: usize,
}

impl Animation {
    pub fn new(frames: Vec<Frame>, fps: u32) -> Self {
        Self {
            count: frames.len(),
            current: 0,
            frames: Arc::new(frames),
            scheduler: Scheduler::new(fps),
            on_pause: None,
            running: true,
        }
    }
    pub const fn count(&self) -> usize {
        self.count
    }
    pub const fn current(&self) -> usize {
        self.current
    }
    pub fn next(&mut self) {
        if !self.running {
            return;
        }
        if self.scheduler.switch_next() {
            self.current += 1;
            if self.current >= self.count {
                self.current = 0;
            }
        }
    }
    pub fn to_node(&self, x: f64, y: f64) -> SceneNode {
        if !self.running && self.on_pause.is_some() {
            self.on_pause.as_ref().map(|i| i.to_node(x, y)).unwrap()
        } else {
            let frame = &self.frames[self.current];
            frame.to_node(x, y)
        }
    }
}
