use super::effect::EffectHandle;
use kira::track::TrackHandle;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Track {
    pub handle: TrackHandle,
    pub effects: HashMap<String, EffectHandle>,
}
