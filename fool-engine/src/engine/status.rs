use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Deserialize)]
pub enum EngineStatus {
    Init,
    Running,
    Pause,
    Exiting,
}

impl EngineStatus {
    pub fn init(&self) -> bool {
        *self == Self::Init
    }
    pub fn running(&self) -> bool {
        *self == Self::Running
    }
    pub fn pause(&self) -> bool {
        *self == Self::Pause
    }
    pub fn exiting(&self) -> bool {
        *self == Self::Exiting
    }
}
