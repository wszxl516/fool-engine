use std::{
    fmt::Display,
    time::{Duration, Instant},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameID {
    id: u64,
    instant: Instant,
}
impl Display for FrameID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
impl FrameID {
    pub fn new() -> Self {
        Self {
            id: 0,
            instant: Instant::now(),
        }
    }
    pub fn elapsed(&self) -> Duration {
        self.instant.elapsed()
    }
    pub fn advance(&mut self) {
        self.instant = Instant::now();
        self.id += 1;
    }
}
impl From<u64> for FrameID {
    fn from(id: u64) -> Self {
        Self {
            id,
            instant: Instant::now(),
        }
    }
}

impl From<FrameID> for u64 {
    fn from(fid: FrameID) -> Self {
        fid.id
    }
}

impl std::ops::AddAssign<u64> for FrameID {
    fn add_assign(&mut self, rhs: u64) {
        self.id = self.id.wrapping_add(rhs);
    }
}
