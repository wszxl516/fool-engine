use mlua::{FromLua, LuaSerdeExt, Value};
use nannou_egui::egui::{
    ecolor::Color32,
    epaint::{Rounding, Shadow, Stroke},
    style::Margin,
};
use serde::{Deserialize, Serialize};

use crate::graphics::types::LuaColor;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct LuaShadow {
    #[serde(default)]
    pub extrusion: f32,
    #[serde(default)]
    pub color: LuaColor,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct LuaStroke {
    #[serde(default)]
    pub width: f32,
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
    pub rounding: Rounding,
    #[serde(default)]
    pub shadow: LuaShadow,
    #[serde(default)]
    pub fill: LuaColor,
    #[serde(default)]
    pub stroke: LuaStroke,
}

impl Into<nannou_egui::egui::Frame> for LuaFrame {
    fn into(self) -> nannou_egui::egui::Frame {
        nannou_egui::egui::Frame {
            inner_margin: self.inner_margin,
            outer_margin: self.outer_margin,
            rounding: self.rounding,
            shadow: Shadow {
                extrusion: self.shadow.extrusion,
                color: Color32::from_rgba_premultiplied(
                    self.shadow.color.r,
                    self.shadow.color.g,
                    self.shadow.color.b,
                    self.shadow.color.a,
                ),
            },
            fill: Color32::from_rgba_premultiplied(
                self.fill.r,
                self.fill.g,
                self.fill.b,
                self.fill.a,
            ),
            stroke: Stroke {
                width: self.stroke.width,
                color: Color32::from_rgba_premultiplied(
                    self.stroke.color.r,
                    self.stroke.color.g,
                    self.stroke.color.b,
                    self.stroke.color.a,
                ),
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
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl FromLua for LuaUIConfig {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
