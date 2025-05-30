use egui::{
    epaint::{CornerRadius, Margin},
    Frame, Shadow, Stroke,
};
use mlua::{FromLua, LuaSerdeExt, Value};
use serde::{Deserialize, Serialize};

use crate::graphics::types::LuaColor;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct LuaShadow {
    #[serde(default)]
    pub offset: [i8; 2],
    #[serde(default)]
    pub blur: u8,
    #[serde(default)]
    pub spread: u8,
    #[serde(default)]
    pub color: LuaColor,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct LuaFrame {
    #[serde(default)]
    pub inner_margin: Margin,
    #[serde(default)]
    pub outer_margin: Margin,
    #[serde(default)]
    pub rounding: CornerRadius,
    #[serde(default)]
    pub shadow: LuaShadow,
    #[serde(default)]
    pub fill: LuaColor,
    #[serde(default)]
    pub stroke_width: f32,
    #[serde(default)]
    pub stroke_color: LuaColor,
}

impl Into<Frame> for LuaFrame {
    fn into(self) -> Frame {
        Frame {
            inner_margin: self.inner_margin,
            fill: self.fill.into(),
            stroke: Stroke {
                width: self.stroke_width,
                color: self.stroke_color.into(),
            },
            corner_radius: self.rounding,
            outer_margin: self.outer_margin,
            shadow: Shadow {
                offset: self.shadow.offset,
                blur: self.shadow.blur,
                spread: self.shadow.spread,
                color: self.shadow.color.into(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LuaUIConfig {
    pub title: String,
    #[serde(default)]
    pub collapsible: bool,
    #[serde(default)]
    pub constrain: bool,
    #[serde(default)]
    pub default_open: bool,
    #[serde(default)]
    pub drag_to_scroll: bool,
    #[serde(default)]
    pub resizable: bool,
    #[serde(default)]
    pub title_bar: bool,
    #[serde(default)]
    pub movable: bool,
    #[serde(default)]
    pub frame: LuaFrame,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub w: f32,
    #[serde(default)]
    pub h: f32,
}

impl FromLua for LuaUIConfig {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
