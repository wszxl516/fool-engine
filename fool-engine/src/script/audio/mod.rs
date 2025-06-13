use crate::map2lua_error;
use fool_audio::{AudioSystem, EffectConfig};
use mlua::{FromLua, IntoLua, LuaSerdeExt, UserData, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaEffectConfig(EffectConfig);
impl FromLua for LuaEffectConfig {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
impl IntoLua for LuaEffectConfig {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        lua.to_value(&self)
    }
}
#[derive(Clone)]
pub struct LuaAudio(pub AudioSystem);

impl UserData for LuaAudio {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "add_group",
            |_lua,
             this,
             (name, volume, persist, effects): (
                String,
                f32,
                bool,
                Option<HashMap<String, LuaEffectConfig>>,
            )| {
                let effects = if let Some(effects) = &effects {
                    effects.iter().map(|(k, v)| (k, v.0.clone())).collect()
                } else {
                    Default::default()
                };
                map2lua_error!(
                    this.0.add_group(name, volume, persist, effects),
                    "LuaAudio add_group"
                )?;
                Ok(())
            },
        );
        methods.add_method(
            "play",
            |_lua,
             this,
             (group, audio, volume, panning, position): (
                String,
                String,
                Option<f32>,
                Option<f32>,
                Option<f64>,
            )| {
                map2lua_error!(
                    this.0.play(group, audio, volume, panning, position),
                    "LuaAudio play"
                )?;
                Ok(())
            },
        );
        methods.add_method(
            "pause",
            |_lua, this, (group, audio, duration): (String, String, u64)| {
                map2lua_error!(this.0.pause(group, audio, duration), "LuaAudio pause")?;
                Ok(())
            },
        );

        methods.add_method(
            "resume",
            |_lua, this, (group, audio, duration): (String, String, u64)| {
                map2lua_error!(this.0.resume(group, audio, duration), "LuaAudio resume")?;
                Ok(())
            },
        );

        methods.add_method(
            "stop",
            |_lua, this, (group, audio, duration): (String, String, u64)| {
                map2lua_error!(this.0.stop(group, audio, duration), "LuaAudio stop")?;
                Ok(())
            },
        );
        methods.add_method(
            "seek_by",
            |_lua, this, (group, audio, amount): (String, String, f64)| {
                map2lua_error!(this.0.seek_by(group, audio, amount), "LuaAudio seek_by")?;
                Ok(())
            },
        );
        methods.add_method(
            "seek_to",
            |_lua, this, (group, audio, position): (String, String, f64)| {
                map2lua_error!(this.0.seek_to(group, audio, position), "LuaAudio seek_to")?;
                Ok(())
            },
        );

        methods.add_method(
            "set_volume",
            |_lua, this, (group, audio, volume, duration): (String, String, f32, u64)| {
                map2lua_error!(
                    this.0.set_volume(group, audio, duration, volume),
                    "LuaAudio set_volume"
                )?;
                Ok(())
            },
        );

        methods.add_method(
            "set_panning",
            |_lua, this, (group, audio, panning, duration): (String, String, f32, u64)| {
                map2lua_error!(
                    this.0.set_panning(group, audio, duration, panning),
                    "LuaAudio panning"
                )?;
                Ok(())
            },
        );

        methods.add_method("state", |_lua, this, (group, audio): (String, String)| {
            if let Some(state) = this.0.state(group, audio) {
                Ok(Some(format!("{:?}", state)))
            } else {
                Ok(None)
            }
        });

        methods.add_method(
            "set_effect",
            |_lua,
             this,
             (group,effect, config,tween): (
                String,
                String,
                LuaEffectConfig,
                Option<u64>,
            )| {
                map2lua_error!(
                    this.0.set_effect(group, effect, config.0,tween),
                    "LuaAudio set_effect"
                )?;
                Ok(())
            },
        );
        methods.add_method("pause_all", |_lua, this, duration: u64| {
            this.0.pause_all(duration);
            Ok(())
        });

        methods.add_method("resume_all", |_lua, this, duration: u64| {
            this.0.resume_all(duration);
            Ok(())
        });
        methods.add_method(
            "set_volume_all",
            |_lua, this, (volume, duration): (f32, u64)| {
                this.0.set_volume_all(volume, duration);
                Ok(())
            },
        );
        methods.add_method("stop_all", |_lua, this, duration: u64| {
            this.0.stop_all(duration);
            Ok(())
        });
    }
}
