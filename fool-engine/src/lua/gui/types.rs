use crate::lua::types::{LuaPoint, LuaSize};

use super::super::graphics::types::LuaColor;
use egui::{epaint::text::TextWrapMode, FontId, TextStyle};
use egui::{
    epaint::{CornerRadius, Margin},
    Frame, Shadow, Stroke,
};
use mlua::{FromLua, IntoLua, Lua, LuaSerdeExt, Result as LuaResult, Value};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, collections::HashMap};
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
    #[serde(default)]
    pub bg_img: Option<String>,
    #[serde(default)]
    pub bg_img_color: Option<LuaColor>,
}

impl FromLua for LuaUIConfig {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LuaGuiStyle {
    pub text: HashMap<std::string::String, f32>,
    pub dark: bool,
    #[serde(default)]
    pub animation_time: f32,
    #[serde(default)]
    pub wrap: Option<TextWrapMode>,
    #[serde(default)]
    pub noninteractive_fg_color: Option<LuaColor>,
    #[serde(default)]
    pub hovered_fg_color: Option<LuaColor>,
    #[serde(default)]
    pub active_fg_color: Option<LuaColor>,
    #[serde(default)]
    pub inactive_fg_color: Option<LuaColor>,
    #[serde(default)]
    pub open_fg_color: Option<LuaColor>,
}
impl LuaGuiStyle {
    pub fn text_style(&self) -> BTreeMap<TextStyle, FontId> {
        self.text
            .iter()
            .map(|s| {
                let name = match s.0.as_str() {
                    "Small" => TextStyle::Small,
                    "Body" => TextStyle::Body,
                    "Monospace" => TextStyle::Monospace,
                    "Button" => TextStyle::Button,
                    "Heading" => TextStyle::Heading,
                    _ => TextStyle::Name(s.0.as_str().into()),
                };
                (name, FontId::proportional(*s.1))
            })
            .collect::<BTreeMap<TextStyle, FontId>>()
    }
}

impl IntoLua for LuaGuiStyle {
    fn into_lua(self, lua: &Lua) -> LuaResult<mlua::Value> {
        lua.to_value(&self)
    }
}
impl FromLua for LuaGuiStyle {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rotate {
    pub angle: f32,
    pub origin: LuaPoint<f32>,
}
impl FromLua for Rotate {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UV {
    pub min: LuaPoint<f32>,
    pub max: LuaPoint<f32>,
}
impl FromLua for UV {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageButtonConfig {
    #[serde(default)]
    pub img: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub show_loading_spinner: Option<bool>,
    #[serde(default)]
    pub img_bg_fill: Option<LuaColor>,
    #[serde(default)]
    pub img_max_size: Option<LuaSize<f32>>,
    #[serde(default)]
    pub img_rotate: Option<Rotate>,
    #[serde(default)]
    pub frame: Option<bool>,
    #[serde(default)]
    pub tint: Option<LuaColor>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub corner_radius: Option<CornerRadius>,
    #[serde(default)]
    pub uv: Option<UV>,
    #[serde(default)]
    pub sense: Option<String>,
}

impl FromLua for ImageButtonConfig {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
