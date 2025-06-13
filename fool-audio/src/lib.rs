use fool_resource::{Resource, SharedData};
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Tween,
    sound::{
        PlaybackState,
        static_sound::{StaticSoundData, StaticSoundHandle},
    },
    track::TrackBuilder,
};
mod effect;
mod group;
use dashmap::DashMap;
pub use effect::{EffectConfig, EffectHandle};
pub use group::Track;
use parking_lot::Mutex;
use std::{collections::HashMap, io::Cursor, sync::Arc, time::Duration};

#[derive(Clone)]
pub struct AudioSystem {
    pub manager: Arc<AudioManager>,
    pub groups: Arc<DashMap<String, Track>>,
    pub master: Arc<Mutex<Track>>,
    pub musics: Arc<DashMap<MusicId, StaticSoundHandle>>,
    pub resource: Resource<String, SharedData>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct MusicId {
    pub track: String,
    pub music: String,
}

impl AudioSystem {
    pub fn new(resource: Resource<String, SharedData>) -> anyhow::Result<Self> {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
        let master = manager.add_sub_track(TrackBuilder::default())?;
        Ok(Self {
            manager: Arc::new(manager),
            groups: Default::default(),
            master: Arc::new(Mutex::new(Track {
                handle: master,
                effects: Default::default(),
            })),
            resource,
            musics: Default::default(),
        })
    }
    pub fn add_group(
        &self,
        name: impl Into<String>,
        volume: f32,
        persist: bool,
        effects: HashMap<impl Into<String>, EffectConfig>,
    ) -> anyhow::Result<()> {
        let mut track = TrackBuilder::new()
            .volume(volume)
            .persist_until_sounds_finish(persist);
        let mut e = HashMap::default();
        for (n, effect) in effects {
            let (ef, ha) = effect.build();
            track.add_built_effect(ef);
            e.insert(n.into(), ha);
        }
        let handle = self.master.lock().handle.add_sub_track(track)?;
        self.groups
            .insert(name.into(), Track { handle, effects: e });
        Ok(())
    }
    pub fn set_effect(
        &self,
        group: impl Into<String>,
        effect: impl Into<String>,
        config: EffectConfig,
        tween: Option<u64>,
    ) -> anyhow::Result<()> {
        let group = group.into();
        let effect = effect.into();
        if let Some(mut track) = self.groups.get_mut(&group) {
            if let Some(effect) = track.effects.get_mut(&effect) {
                effect.set(config, tween);
                Ok(())
            } else {
                Err(anyhow::anyhow!("effect {} Not Found!", effect))
            }
        } else {
            Err(anyhow::anyhow!("group {} Not Found!", effect))
        }
    }
    pub fn pause_all(&self, duration: u64) {
        let tween = Tween {
            start_time: Default::default(),
            duration: Duration::from_millis(duration),
            easing: kira::Easing::Linear,
        };
        self.master.lock().handle.pause(tween);
    }
    pub fn resume_all(&self, duration: u64) {
        let tween = Tween {
            start_time: Default::default(),
            duration: Duration::from_millis(duration),
            easing: kira::Easing::Linear,
        };
        self.master.lock().handle.resume(tween);
    }
    pub fn set_volume_all(&self, volume: f32, duration: u64) {
        let tween = Tween {
            start_time: Default::default(),
            duration: Duration::from_millis(duration),
            easing: kira::Easing::Linear,
        };
        self.master.lock().handle.set_volume(volume, tween);
    }
    pub fn stop_all(&self, duration: u64) {
        let tween = Tween {
            start_time: Default::default(),
            duration: Duration::from_millis(duration),
            easing: kira::Easing::Linear,
        };
        for mut music in self.musics.iter_mut() {
            let (_name, handle) = music.pair_mut();
            handle.stop(tween);
        }
    }
    pub fn play(
        &self,
        group: impl Into<String>,
        music: impl Into<String>,
        volume: Option<f32>,
        panning: Option<f32>,
        position: Option<f64>,
    ) -> anyhow::Result<()> {
        let track = group.into();
        let music = music.into();
        match self.groups.get_mut(&track) {
            Some(mut t) => {
                if let Ok(audio) = self.resource.get(&music) {
                    let mut sound_data = StaticSoundData::from_cursor(Cursor::new(audio.clone()))?;
                    if let Some(v) = volume {
                        sound_data = sound_data.volume(v);
                    }
                    if let Some(v) = panning {
                        sound_data = sound_data.panning(v);
                    }
                    if let Some(v) = position {
                        sound_data = sound_data.start_position(v);
                    }
                    let handle = t.handle.play(sound_data)?;
                    self.musics.insert(
                        MusicId {
                            track: track.clone(),
                            music: music.clone(),
                        },
                        handle,
                    );
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("audio {} Not Found!", music))
                }
            }
            None => Err(anyhow::anyhow!("group {} Not Found!", track)),
        }
    }
    pub fn pause(
        &self,
        group: impl Into<String>,
        music: impl Into<String>,
        duration: u64,
    ) -> anyhow::Result<()> {
        let track = group.into();
        let music = music.into();
        match self.musics.get_mut(&MusicId {
            track: track.clone(),
            music: music.clone(),
        }) {
            Some(mut t) => {
                let tween = Tween {
                    start_time: Default::default(),
                    duration: Duration::from_millis(duration),
                    easing: kira::Easing::Linear,
                };
                t.pause(tween);
                Ok(())
            }
            None => Err(anyhow::anyhow!(
                "audio {} of group {} Not Found!",
                music,
                track
            )),
        }
    }
    pub fn resume(
        &self,
        group: impl Into<String>,
        music: impl Into<String>,
        duration: u64,
    ) -> anyhow::Result<()> {
        let track = group.into();
        let music = music.into();
        match self.musics.get_mut(&MusicId {
            track: track.clone(),
            music: music.clone(),
        }) {
            Some(mut t) => {
                let tween = Tween {
                    start_time: Default::default(),
                    duration: Duration::from_millis(duration),
                    easing: kira::Easing::Linear,
                };
                t.resume(tween);
                Ok(())
            }
            None => Err(anyhow::anyhow!(
                "audio {} of group {} Not Found!",
                music,
                track
            )),
        }
    }

    pub fn stop(
        &self,
        group: impl Into<String>,
        music: impl Into<String>,
        duration: u64,
    ) -> anyhow::Result<()> {
        let track = group.into();
        let music = music.into();
        match self.musics.get_mut(&MusicId {
            track: track.clone(),
            music: music.clone(),
        }) {
            Some(mut t) => {
                let tween = Tween {
                    start_time: Default::default(),
                    duration: Duration::from_millis(duration),
                    easing: kira::Easing::Linear,
                };
                t.stop(tween);
                Ok(())
            }
            None => Err(anyhow::anyhow!(
                "audio {} of group {} Not Found!",
                music,
                track
            )),
        }
    }
    pub fn seek_by(
        &self,
        group: impl Into<String>,
        music: impl Into<String>,
        amount: f64,
    ) -> anyhow::Result<()> {
        let track = group.into();
        let music = music.into();
        match self.musics.get_mut(&MusicId {
            track: track.clone(),
            music: music.clone(),
        }) {
            Some(mut t) => {
                t.seek_by(amount);
                Ok(())
            }
            None => Err(anyhow::anyhow!(
                "audio {} of group {} Not Found!",
                music,
                track
            )),
        }
    }
    pub fn seek_to(
        &self,
        track: impl Into<String>,
        group: impl Into<String>,
        position: f64,
    ) -> anyhow::Result<()> {
        let track = track.into();
        let music = group.into();
        match self.musics.get_mut(&MusicId {
            track: track.clone(),
            music: music.clone(),
        }) {
            Some(mut t) => {
                t.seek_to(position);
                Ok(())
            }
            None => Err(anyhow::anyhow!(
                "audio {} of group {} Not Found!",
                music,
                track
            )),
        }
    }
    pub fn set_volume(
        &self,
        group: impl Into<String>,
        music: impl Into<String>,
        duration: u64,
        volume: f32,
    ) -> anyhow::Result<()> {
        let track = group.into();
        let music = music.into();
        match self.musics.get_mut(&MusicId {
            track: track.clone(),
            music: music.clone(),
        }) {
            Some(mut t) => {
                let tween = Tween {
                    start_time: Default::default(),
                    duration: Duration::from_millis(duration),
                    easing: kira::Easing::Linear,
                };
                t.set_volume(volume, tween);
                Ok(())
            }
            None => Err(anyhow::anyhow!(
                "audio {} of group {} Not Found!",
                music,
                track
            )),
        }
    }
    pub fn set_panning(
        &self,
        group: impl Into<String>,
        music: impl Into<String>,
        duration: u64,
        panning: f32,
    ) -> anyhow::Result<()> {
        let track = group.into();
        let music = music.into();
        match self.musics.get_mut(&MusicId {
            track: track.clone(),
            music: music.clone(),
        }) {
            Some(mut t) => {
                let tween = Tween {
                    start_time: Default::default(),
                    duration: Duration::from_millis(duration),
                    easing: kira::Easing::Linear,
                };
                t.set_panning(panning, tween);
                Ok(())
            }
            None => Err(anyhow::anyhow!(
                "audio {} of group {} Not Found!",
                music,
                track
            )),
        }
    }
    pub fn state(
        &self,
        group: impl Into<String>,
        music: impl Into<String>,
    ) -> Option<PlaybackState> {
        let track = group.into();
        let music = music.into();
        match self.musics.get_mut(&MusicId {
            track: track.clone(),
            music: music.clone(),
        }) {
            Some(t) => {
                let state = t.state();
                Some(state)
            }
            None => None,
        }
    }
}
