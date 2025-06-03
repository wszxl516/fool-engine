use fool_graphics::canvas::{SceneNode, SceneNodeKind};
use mlua::{FromLua, IntoLua, LuaSerdeExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LuaShape(pub SceneNodeKind);

impl FromLua for LuaShape {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
impl IntoLua for LuaShape {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LuaScene(pub SceneNode);

impl FromLua for LuaScene {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
impl IntoLua for LuaScene {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}
