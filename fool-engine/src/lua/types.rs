use egui::Pos2;
use mlua::{FromLua, IntoLua, Lua, LuaSerdeExt, Result as LuaResult, UserData};
use rapier2d::prelude::*;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LuaSize<T> {
    #[serde(default)]
    pub w: T,
    #[serde(default)]
    pub h: T,
}

impl<T: Sized + FromLua + DeserializeOwned + Serialize + Default> FromLua for LuaSize<T> {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}

impl<T: IntoLua + Copy> UserData for LuaSize<T> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("w", |_lua, this| Ok(this.w));
        fields.add_field_method_get("h", |_lua, this| Ok(this.h));
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LuaPoint<T> {
    pub x: T,
    pub y: T,
}
impl Into<Point<f32>> for LuaPoint<f32> {
    fn into(self) -> Point<f32> {
        Point::new(self.x, self.y)
    }
}

impl Into<egui::Pos2> for LuaPoint<f32> {
    fn into(self) -> egui::Pos2 {
        Pos2::new(self.x, self.y)
    }
}
impl<T: Sized + DeserializeOwned> FromLua for LuaPoint<T> {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}
impl<T: IntoLua + Copy> UserData for LuaPoint<T> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_lua, this| Ok(this.x));
        fields.add_field_method_get("y", |_lua, this| Ok(this.y));
    }
}
