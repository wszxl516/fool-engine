#![allow(dead_code)]
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Scheduler {
    frame_interval: Duration,
    pub next_frame_time: Instant,
    pub running: bool,
}
impl Default for Scheduler {
    fn default() -> Self {
        Self::new(30)
    }
}
impl Scheduler {
    pub fn new(fps: u32) -> Self {
        let frame_interval = Duration::from_secs_f64(1.0 / fps as f64);
        let now = Instant::now();
        Self {
            frame_interval,
            next_frame_time: now + frame_interval,
            running: true,
        }
    }

    fn advance(&mut self) {
        self.next_frame_time += self.frame_interval;
    }
    pub fn reset(&mut self) {
        let now = Instant::now();
        self.next_frame_time = now + self.frame_interval;
    }
    pub fn pause(&mut self) {
        self.running = false;
    }
    pub fn resume(&mut self) {
        if !self.running {
            self.running = true;
            self.reset();
        }
    }
    pub fn switch_next(&mut self) -> bool {
        if !self.running {
            return false;
        }
        let mut redraw = false;
        let now = std::time::Instant::now();
        if now >= self.next_frame_time {
            redraw = true;
            self.advance();
        }
        return redraw;
    }
}
