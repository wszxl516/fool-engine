use kira::{
    Panning, Tween, Value,
    effect::{
        Effect, EffectBuilder,
        compressor::{CompressorBuilder, CompressorHandle},
        delay::{DelayBuilder, DelayHandle},
        distortion::{DistortionBuilder, DistortionHandle, DistortionKind},
        eq_filter::{EqFilterBuilder, EqFilterHandle, EqFilterKind},
        filter::{FilterBuilder, FilterHandle, FilterMode},
        panning_control::{PanningControlBuilder, PanningControlHandle},
        reverb::{ReverbBuilder, ReverbHandle},
        volume_control::{VolumeControlBuilder, VolumeControlHandle},
    },
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug)]
pub enum EffectHandle {
    CompressorHandle(CompressorHandle),
    DelayHandle(DelayHandle),
    DistortionHandle(DistortionHandle),
    EqFilterHandle(EqFilterHandle),
    FilterHandle(FilterHandle),
    PanningControlHandle(PanningControlHandle),
    ReverbHandle(ReverbHandle),
    VolumeControlHandle(VolumeControlHandle),
}
impl EffectHandle {
    pub fn set(&mut self, config: EffectConfig, tween: Option<u64>) {
        let tween = tween.unwrap_or(100);
        match config {
            EffectConfig::Compressor {
                attack_duration,
                makeup_gain,
                mix,
                ratio,
                release_duration,
                threshold,
            } => {
                if let EffectHandle::CompressorHandle(h) = self {
                    if let Some(attack_duration) = attack_duration {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_attack_duration(Duration::from_millis(attack_duration), tween);
                    }
                    if let Some(makeup_gain) = makeup_gain {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_makeup_gain(makeup_gain, tween);
                    }
                    if let Some(mix) = mix {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_mix(mix, tween);
                    }
                    if let Some(ratio) = ratio {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_ratio(ratio, tween);
                    }
                    if let Some(release_duration) = release_duration {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_release_duration(Duration::from_millis(release_duration), tween);
                    }
                    if let Some(threshold) = threshold {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_threshold(threshold, tween);
                    }
                }
            }
            EffectConfig::Delay {
                delay_time: _,
                feedback,
                mix,
            } => {
                if let EffectHandle::DelayHandle(h) = self {
                    if let Some(feedback) = feedback {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_feedback(feedback, tween);
                    }
                    if let Some(mix) = mix {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_mix(mix, tween);
                    }
                }
            }
            EffectConfig::Distortion { drive, kind, mix } => {
                if let EffectHandle::DistortionHandle(h) = self {
                    if let Some(drive) = drive {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_drive(drive, tween);
                    }
                    if let Some(kind) = kind {
                        h.set_kind(kind);
                    }
                    if let Some(mix) = mix {
                        let tween = Tween {
                            start_time: Default::default(),
                            duration: Duration::from_millis(tween),
                            easing: kira::Easing::Linear,
                        };
                        h.set_mix(mix, tween);
                    }
                }
            }
            EffectConfig::EqFilter {
                kind,
                frequency,
                gain,
                q,
            } => {
                let tween = Tween {
                    start_time: Default::default(),
                    duration: Duration::from_millis(tween),
                    easing: kira::Easing::Linear,
                };
                if let EffectHandle::EqFilterHandle(h) = self {
                    if let Some(kind) = kind {
                        h.set_kind(kind);
                    }
                    if let Some(frequency) = frequency {
                        h.set_frequency(frequency, tween);
                    }
                    if let Some(gain) = gain {
                        h.set_gain(gain, tween);
                    }
                    if let Some(q) = q {
                        h.set_q(q, tween);
                    }
                }
            }
            EffectConfig::PanningControl { panning } => {
                if let EffectHandle::PanningControlHandle(h) = self {
                    let tween = Tween {
                        start_time: Default::default(),
                        duration: Duration::from_millis(tween),
                        easing: kira::Easing::Linear,
                    };
                    h.set_panning(panning, tween);
                }
            }

            EffectConfig::Filter {
                cutoff,
                mix,
                mode,
                resonance,
            } => {
                if let EffectHandle::FilterHandle(h) = self {
                    let tween = Tween {
                        start_time: Default::default(),
                        duration: Duration::from_millis(tween),
                        easing: kira::Easing::Linear,
                    };
                    if let Some(cutoff) = cutoff {
                        h.set_cutoff(cutoff, tween);
                    }
                    if let Some(mix) = mix {
                        h.set_mix(mix, tween);
                    }
                    if let Some(mode) = mode {
                        h.set_mode(mode);
                    }
                    if let Some(resonance) = resonance {
                        h.set_resonance(resonance, tween);
                    }
                }
            }
            EffectConfig::Reverb {
                damping,
                feedback,
                mix,
                stereo_width,
            } => {
                if let EffectHandle::ReverbHandle(h) = self {
                    let tween = Tween {
                        start_time: Default::default(),
                        duration: Duration::from_millis(tween),
                        easing: kira::Easing::Linear,
                    };
                    if let Some(damping) = damping {
                        h.set_damping(damping, tween);
                    }
                    if let Some(feedback) = feedback {
                        h.set_feedback(feedback, tween);
                    }
                    if let Some(mix) = mix {
                        h.set_mix(mix, tween);
                    }
                    if let Some(stereo_width) = stereo_width {
                        h.set_stereo_width(stereo_width, tween);
                    }
                }
            }
            EffectConfig::VolumeControl { volume } => {
                let tween = Tween {
                    start_time: Default::default(),
                    duration: Duration::from_millis(tween),
                    easing: kira::Easing::Linear,
                };
                if let EffectHandle::VolumeControlHandle(h) = self {
                    h.set_volume(volume, tween);
                }
            }
        }
    }
}
#[macro_export]
macro_rules! apply_if_some {
    ($target:ident,$method:ident, $field:expr) => {
        if let Some(val) = $field {
            $target = $target.$method(val);
        }
    };
    ($target:ident, $method:ident, $field:expr, $transform:expr) => {
        if let Some(val) = &$field {
            $target = $target.$method($transform(val));
        }
    };
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EffectConfig {
    Compressor {
        attack_duration: Option<u64>,
        makeup_gain: Option<f32>,
        mix: Option<f32>,
        ratio: Option<f64>,
        release_duration: Option<u64>,
        threshold: Option<f64>,
    },
    Delay {
        delay_time: Option<u64>,
        feedback: Option<f32>,
        mix: Option<f32>,
    },
    Distortion {
        drive: Option<f32>,
        kind: Option<DistortionKind>,
        mix: Option<f32>,
    },
    EqFilter {
        kind: Option<EqFilterKind>,
        frequency: Option<f64>,
        gain: Option<f32>,
        q: Option<f64>,
    },
    Filter {
        cutoff: Option<f64>,
        mix: Option<f32>,
        mode: Option<FilterMode>,
        resonance: Option<f64>,
    },
    PanningControl {
        panning: f32,
    },
    Reverb {
        damping: Option<f64>,
        feedback: Option<f64>,
        mix: Option<f32>,
        stereo_width: Option<f64>,
    },
    VolumeControl {
        volume: f32,
    },
}

impl EffectConfig {
    pub fn build(&self) -> (Box<dyn Effect>, EffectHandle) {
        match self {
            Self::Compressor {
                attack_duration,
                makeup_gain,
                mix,
                ratio,
                release_duration,
                threshold,
            } => {
                let mut builder = CompressorBuilder::new();
                apply_if_some!(builder, attack_duration, attack_duration, move |d: &u64| {
                    Duration::from_millis(*d)
                });
                apply_if_some!(builder, makeup_gain, *makeup_gain);
                apply_if_some!(builder, mix, *mix);
                apply_if_some!(builder, ratio, *ratio);
                apply_if_some!(
                    builder,
                    release_duration,
                    release_duration,
                    move |d: &u64| Duration::from_millis(*d)
                );
                apply_if_some!(builder, threshold, *threshold);
                let (effect, handle) = builder.build();
                (effect, EffectHandle::CompressorHandle(handle))
            }
            Self::Delay {
                delay_time,
                feedback,
                mix,
            } => {
                let mut builder = DelayBuilder::new();
                apply_if_some!(builder, mix, *mix);
                apply_if_some!(builder, delay_time, delay_time, move |d: &u64| {
                    Duration::from_millis(*d)
                });
                apply_if_some!(builder, feedback, *feedback);
                let (effect, handle) = builder.build();
                (effect, EffectHandle::DelayHandle(handle))
            }
            Self::Distortion { drive, kind, mix } => {
                let mut builder = DistortionBuilder::new();
                apply_if_some!(builder, drive, *drive);
                apply_if_some!(builder, kind, *kind);
                apply_if_some!(builder, mix, *mix);
                let (effect, handle) = builder.build();
                (effect, EffectHandle::DistortionHandle(handle))
            }
            Self::EqFilter {
                kind,
                frequency,
                gain,
                q,
            } => {
                let builder = EqFilterBuilder::new(
                    kind.unwrap_or(EqFilterKind::Bell),
                    frequency.unwrap_or(1000.0),
                    gain.unwrap_or(0.0),
                    q.unwrap_or(1.0),
                );
                let (effect, handle) = builder.build();
                (effect, EffectHandle::EqFilterHandle(handle))
            }
            Self::Filter {
                cutoff,
                mix,
                mode,
                resonance,
            } => {
                let mut builder = FilterBuilder::new();
                apply_if_some!(builder, cutoff, *cutoff);
                apply_if_some!(builder, mix, *mix);
                apply_if_some!(builder, resonance, *resonance);
                apply_if_some!(builder, mode, *mode);
                let (effect, handle) = builder.build();
                (effect, EffectHandle::FilterHandle(handle))
            }
            Self::PanningControl { panning } => {
                let (effect, handle) =
                    PanningControlBuilder(Value::Fixed(Panning(*panning))).build();
                (effect, EffectHandle::PanningControlHandle(handle))
            }
            Self::Reverb {
                damping,
                feedback,
                mix,
                stereo_width,
            } => {
                let mut builder = ReverbBuilder::new();
                apply_if_some!(builder, mix, *mix);
                apply_if_some!(builder, damping, *damping);
                apply_if_some!(builder, feedback, *feedback);
                apply_if_some!(builder, stereo_width, *stereo_width);
                let (effect, handle) = builder.build();
                (effect, EffectHandle::ReverbHandle(handle))
            }
            Self::VolumeControl { volume } => {
                let (effect, handle) = VolumeControlBuilder::new(*volume).build();
                (effect, EffectHandle::VolumeControlHandle(handle))
            }
        }
    }
}
