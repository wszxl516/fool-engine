pub use super::super::script::types::LuaPoint;
use mlua::{FromLua, IntoLua, Lua, LuaSerdeExt, Result as LuaResult, UserData, Value};
use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Shape2D {
    Cuboid {
        width: f32,
        height: f32,
    },
    Ball {
        radius: f32,
    },
    CapsuleY {
        height: f32,
        radius: f32,
    },
    CapsuleX {
        width: f32,
        radius: f32,
    },
    RoundCuboid {
        width: f32,
        height: f32,
        border_radius: f32,
    },
    Triangle {
        a: LuaPoint<f32>,
        b: LuaPoint<f32>,
        c: LuaPoint<f32>,
    },
    Convex {
        points: Vec<LuaPoint<f32>>,
    },
}
impl Default for Shape2D {
    fn default() -> Self {
        Shape2D::Cuboid {
            width: 10.0,
            height: 10.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaRigidBody(pub RigidBody);
impl UserData for LuaRigidBody {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("pos", |_lua, this| {
            let pos = this.0.translation();
            Ok(LuaPoint { x: pos.x, y: pos.y })
        });
        fields.add_field_method_get("angle", |_lua, this| {
            let angle = this.0.rotation().angle();
            Ok(angle)
        });
        fields.add_field_method_get("linvel", |_lua, this| {
            let linvel = this.0.linvel();
            Ok(LuaPoint {
                x: linvel.x,
                y: linvel.y,
            })
        });
        fields.add_field_method_get("angvel", |_lua, this| {
            let angvel = this.0.angvel();
            Ok(angvel)
        });
        fields.add_field_method_get("mass", |_lua, this| {
            let mass = this.0.mass();
            Ok(mass)
        });
        fields.add_field_method_get("is_fixed", |_lua, this| {
            let is_fixed = this.0.is_fixed();
            Ok(is_fixed)
        });
        fields.add_field_method_get("user_data", |_lua, this| {
            let user_data = this.0.user_data;
            Ok(user_data)
        });
    }
}
impl FromLua for LuaRigidBody {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyData {
    #[serde(default)]
    pub user_data: u128,
    #[serde(default)]
    pub position: LuaPoint<f32>,
    #[serde(default)]
    pub shape: Shape2D,
    #[serde(default = "default_body_type")]
    pub body_type: RigidBodyType,
    #[serde(default)]
    pub rotation: Option<f32>,
    #[serde(default)]
    pub linear_damping: f32,
    #[serde(default)]
    pub angular_damping: f32,
    #[serde(default = "default_gravity_scale")]
    pub gravity_scale: f32,
    #[serde(default = "default_mass")]
    pub additional_mass: f32,
    #[serde(default = "default_mass")]
    pub mass: f32,
    #[serde(default)]
    pub can_sleep: bool,
    #[serde(default)]
    pub sleeping: bool,
    #[serde(default = "default_restitution")]
    pub restitution: f32,
    #[serde(default = "default_friction")]
    pub friction: f32,
    #[serde(default = "default_density")]
    pub density: f32,
    #[serde(default)]
    pub is_sensor: bool,
    #[serde(default)]
    pub active_events: LuaActiveEvents,
    #[serde(default)]
    pub active_hooks: LuaActiveHooks,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[repr(transparent)]
pub struct LuaActiveHooks(String);
impl Into<ActiveHooks> for LuaActiveHooks {
    fn into(self) -> ActiveHooks {
        match self.0.as_str() {
            "filter_contact_pairs" => ActiveHooks::FILTER_CONTACT_PAIRS,
            "filter_intersection_pair" => ActiveHooks::FILTER_INTERSECTION_PAIR,
            "all" => ActiveHooks::FILTER_CONTACT_PAIRS | ActiveHooks::FILTER_INTERSECTION_PAIR,
            _ => ActiveHooks::empty(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[repr(transparent)]
pub struct LuaActiveEvents(String);
impl Into<ActiveEvents> for LuaActiveEvents {
    fn into(self) -> ActiveEvents {
        match self.0.as_str() {
            "collision_events" => ActiveEvents::COLLISION_EVENTS,
            "contact_force_events" => ActiveEvents::CONTACT_FORCE_EVENTS,
            "all" => ActiveEvents::all(),
            _ => ActiveEvents::empty(),
        }
    }
}
impl FromLua for Shape2D {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        let shape: Shape2D = lua.from_value(value)?;
        Ok(shape)
    }
}

impl FromLua for BodyData {
    fn from_lua(value: Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}

const fn default_body_type() -> RigidBodyType {
    RigidBodyType::Dynamic
}
const fn default_friction() -> f32 {
    0.5
}
const fn default_density() -> f32 {
    1.0
}
const fn default_restitution() -> f32 {
    0.5
}
const fn default_gravity_scale() -> f32 {
    10.0
}
const fn default_mass() -> f32 {
    10.0
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LuaRigidBodyHandle(pub RigidBodyHandle);
impl FromLua for LuaRigidBodyHandle {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
impl IntoLua for LuaRigidBodyHandle {
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        lua.to_value(&self)
    }
}
// impl UserData for LuaRigidBodyHandle {}
