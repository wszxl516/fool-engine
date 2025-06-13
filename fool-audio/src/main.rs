use fool_resource::Resource;
use std::collections::HashMap;

use fool_audio::{AudioSystem, EffectConfig};

const OGG0: &[u8] = include_bytes!("../../assets/audio/jump.mp3");
const OGG1: &[u8] = include_bytes!("../../assets/audio/bgm.mp3");
fn main() -> anyhow::Result<()> {
    let res = Resource::empty();
    res.load("000", OGG0);
    res.load("001", OGG1);
    let a = AudioSystem::new(res)?;
    let mut effects = HashMap::new();
    effects.insert(
        "test1",
        EffectConfig::Compressor {
            attack_duration: Some(1),
            makeup_gain: Some(1.0),
            mix: Some(1.0),
            ratio: Some(1.0),
            release_duration: Some(1),
            threshold: Some(1.0),
        },
    );
    a.add_group("test", 0.5, true, effects)?;
    a.set_volume_all(0.1, 0);
    a.play("test", "000", Some(-6.8), None, None)?;
    a.play("test", "001", Some(-6.8), None, None)?;
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
