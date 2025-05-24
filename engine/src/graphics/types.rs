pub use super::super::lua::types::{LuaPoint, LuaSize};
use lyon_tessellation::StrokeOptions;
use mlua::{FromLua, IntoLua, Lua, LuaSerdeExt, Result as LuaResult};
use nannou::{
    color::{encoding::Linear, Alpha, LinSrgba},
    draw::{primitive::polygon::PolygonOptions, properties::spatial::position::Properties},
    geom::pt3,
    text::{Align, Justify, Wrap},
};
use nannou_egui::egui::{Color32, FontId, TextStyle};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, collections::HashMap};

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
pub type LinearRrgba = Alpha<nannou::prelude::rgb::Rgb<Linear<nannou::color::encoding::Srgb>>, f32>;
impl Into<LinearRrgba> for LuaColor {
    fn into(self) -> LinearRrgba {
        let color = LinSrgba::from_components((
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ));
        color
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
impl Into<Align> for LuaTextAlign {
    fn into(self) -> Align {
        match self {
            LuaTextAlign::Start => Align::Start,
            LuaTextAlign::Middle => Align::Middle,
            LuaTextAlign::End => Align::End,
        }
    }
}
impl Into<Justify> for LuaTextAlign {
    fn into(self) -> Justify {
        match self {
            LuaTextAlign::Start => Justify::Left,
            LuaTextAlign::Middle => Justify::Center,
            LuaTextAlign::End => Justify::Right,
        }
    }
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

impl Into<Wrap> for LuaTextWrap {
    fn into(self) -> Wrap {
        match self {
            LuaTextWrap::Character => Wrap::Character,
            LuaTextWrap::Whitespace => Wrap::Whitespace,
        }
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
pub struct LuaPolygonOptions {
    #[serde(default)]
    pub position: Position,
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    #[serde(default)]
    pub no_fill: bool,
    #[serde(default)]
    pub stroke_color: Option<LuaColor>,
    #[serde(default)]
    pub color: Option<LuaColor>,
    #[serde(default)]
    pub stroke: Option<StrokeOptions>,
    #[serde(default)]
    pub radians: LuaRadians,
}
impl Into<PolygonOptions> for LuaPolygonOptions {
    fn into(self) -> PolygonOptions {
        PolygonOptions {
            position: Properties {
                point: pt3(self.position.x, self.position.y, self.position.z),
            },
            orientation: Default::default(),
            no_fill: self.no_fill,
            stroke_color: self.stroke_color.map(|s| s.into()),
            color: self.color.map(|s| s.into()),
            stroke: self.stroke,
        }
    }
}
impl FromLua for LuaPolygonOptions {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LuaGuiStyle {
    pub text: HashMap<std::string::String, f32>,
    pub dark: bool,
    #[serde(default)]
    pub animation_time: f32,
    #[serde(default)]
    pub wrap: Option<bool>,
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
