pub use super::super::types::{LuaPoint, LuaSize};
use egui::Color32;
use mlua::{FromLua, IntoLua, Lua, LuaSerdeExt, Result as LuaResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct LuaColor {
    #[serde(default)]
    pub r: u8,
    #[serde(default)]
    pub g: u8,
    #[serde(default)]
    pub b: u8,
    #[serde(default)]
    pub a: u8,
}

impl Into<Color32> for LuaColor {
    fn into(self) -> Color32 {
        Color32::from_rgba_premultiplied(self.r, self.g, self.b, self.a)
    }
}

impl IntoLua for LuaColor {
    fn into_lua(self, lua: &Lua) -> LuaResult<mlua::Value> {
        lua.to_value(&self)
    }
}
impl FromLua for LuaColor {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        let v: LuaColor = lua.from_value(value)?;
        Ok(v)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum LuaTextAlign {
    Start,
    #[default]
    Middle,
    End,
}

impl FromLua for LuaTextAlign {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        let align: LuaTextAlign = lua.from_value(value)?;
        Ok(align)
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum LuaTextWrap {
    Character,
    #[default]
    Whitespace,
}
impl FromLua for LuaTextWrap {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        let align: LuaTextWrap = lua.from_value(value)?;
        Ok(align)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaTextLayout {
    #[serde(default)]
    pub line_spacing: f32,
    #[serde(default)]
    pub line_wrap: Option<LuaTextWrap>,
    pub size: Option<LuaSize<f32>>,
    #[serde(default)]
    pub x_align: LuaTextAlign,
    #[serde(default)]
    pub y_align: LuaTextAlign,
    #[serde(default)]
    pub radians: Option<f32>,
    #[serde(default)]
    pub gray: Option<f32>,
}
impl FromLua for LuaTextLayout {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LuaRadians {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
}
impl FromLua for LuaRadians {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}
const fn default_area() -> (f32, f32) {
    (0.0, 1.0)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaTextureLayout {
    #[serde(default)]
    pub radians: LuaRadians,
    #[serde(default = "default_area")]
    pub area_x: (f32, f32),
    #[serde(default = "default_area")]
    pub area_y: (f32, f32),
}

impl FromLua for LuaTextureLayout {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LuaColoredPoint {
    pub p: LuaPoint<f32>,
    pub c: LuaColor,
}
impl FromLua for LuaColoredPoint {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}
