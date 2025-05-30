use std::time::{Duration, Instant};

pub struct Scheduler {
    frame_interval: Duration,
    pub next_frame_time: Instant,
}

impl Scheduler {
    pub fn new(fps: u32) -> Self {
        let frame_interval = Duration::from_secs_f64(1.0 / fps as f64);
        let now = Instant::now();
        Self {
            frame_interval,
            next_frame_time: now + frame_interval,
        }
    }

    pub fn should_redraw(&self) -> bool {
        Instant::now() >= self.next_frame_time
    }
    pub fn advance(&mut self) {
        self.next_frame_time = Instant::now() + self.frame_interval;
    }
    pub fn reset(&mut self) {
        let now = Instant::now();
        self.next_frame_time = now + self.frame_interval;
    }
}
