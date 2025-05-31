#![allow(dead_code)]
use std::time::{Duration, Instant};

use winit::event_loop::{ActiveEventLoop, ControlFlow};

#[derive(Debug)]
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

    pub fn advance(&mut self) {
        self.next_frame_time = Instant::now() + self.frame_interval;
    }
    pub fn reset(&mut self) {
        let now = Instant::now();
        self.next_frame_time = now + self.frame_interval;
    }
    pub fn trigger_redraw(&mut self, event_loop: &ActiveEventLoop) -> bool {
        let mut redraw = false;
        let now = std::time::Instant::now();
        if now >= self.next_frame_time {
            redraw = true;
            self.advance();
        }
        let next = self.next_frame_time;
        let wait = if next > now {
            next
        } else {
            now + std::time::Duration::from_millis(1)
        };
        event_loop.set_control_flow(ControlFlow::WaitUntil(wait));
        return redraw;
    }
}
