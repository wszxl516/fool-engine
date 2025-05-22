use super::LuaRigidBodyHandle;
use mlua::{FromLua, LuaSerdeExt, UserData, Value};
use rapier2d::{
    geometry::CollisionEventFlags,
    pipeline::EventHandler,
    prelude::{ColliderSet, CollisionEvent, ContactPair, RigidBodySet},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LuaCollisionEvent {
    Started {
        b1: LuaRigidBodyHandle,
        b2: LuaRigidBodyHandle,
        sensor: bool,
        removed: bool,
    },
    Stopped {
        b1: LuaRigidBodyHandle,
        b2: LuaRigidBodyHandle,
        sensor: bool,
        removed: bool,
    },
}
impl FromLua for LuaCollisionEvent {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

impl UserData for LuaCollisionEvent {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("started", |lua, this| match this {
            LuaCollisionEvent::Started {
                b1,
                b2,
                sensor,
                removed,
            } => {
                let table = lua.create_table()?;
                table.set("b1", *b1)?;
                table.set("b2", *b2)?;
                table.set("sensor", *sensor)?;
                table.set("removed", *removed)?;

                Ok(Value::Table(table))
            }
            _ => Ok(Value::Nil),
        });
        fields.add_field_method_get("stopped", |lua, this| match this {
            LuaCollisionEvent::Stopped {
                b1,
                b2,
                sensor,
                removed,
            } => {
                let table = lua.create_table()?;
                table.set("b1", *b1)?;
                table.set("b2", *b2)?;
                table.set("sensor", *sensor)?;
                table.set("removed", *removed)?;

                Ok(Value::Table(table))
            }
            _ => Ok(Value::Nil),
        });
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LuaContactForceEvent {
    h1: LuaRigidBodyHandle,
    h2: LuaRigidBodyHandle,
    dt: f32,
    total_force_magnitude: f32,
}

impl FromLua for LuaContactForceEvent {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
impl UserData for LuaContactForceEvent {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("h1", |_lua, this| Ok(this.h1));
        fields.add_field_method_get("h2", |_lua, this| Ok(this.h2));
        fields.add_field_method_get("dt", |_lua, this| Ok(this.dt));
        fields.add_field_method_get("total_force_magnitude", |_lua, this| {
            Ok(this.total_force_magnitude)
        });
    }
}

#[derive(Clone, Default)]
pub struct LuaPhyEventHandler {
    collision_event: Arc<Mutex<Vec<LuaCollisionEvent>>>,
    contact_force_event: Arc<Mutex<Vec<LuaContactForceEvent>>>,
}

impl LuaPhyEventHandler {
    pub fn reset_all(&self) {
        if let Ok(ref mut data) = self.collision_event.lock() {
            data.clear();
        };
        if let Ok(ref mut data) = self.contact_force_event.lock() {
            data.clear();
        };
    }
    pub fn handle_collision_event(&self, f: impl Fn(&LuaCollisionEvent)) {
        if let Ok(data) = self.collision_event.lock() {
            data.iter().for_each(move |e| f(e));
        }
    }
    pub fn handle_contact_force_event(&self, f: impl Fn(&LuaContactForceEvent)) {
        if let Ok(data) = self.contact_force_event.lock() {
            data.iter().for_each(move |e| f(e));
        }
    }
}
impl EventHandler for LuaPhyEventHandler {
    fn handle_collision_event(
        &self,
        _bodies: &rapier2d::prelude::RigidBodySet,
        colliders: &rapier2d::prelude::ColliderSet,
        event: rapier2d::prelude::CollisionEvent,
        _contact_pair: Option<&rapier2d::prelude::ContactPair>,
    ) {
        match event {
            CollisionEvent::Started(handle1, handle2, flags) => {
                if let (Some(rb1), Some(rb2)) = (
                    colliders.get(handle1).and_then(|col| col.parent()),
                    colliders.get(handle2).and_then(|col| col.parent()),
                ) {
                    if let Ok(ref mut data) = self.collision_event.lock() {
                        data.push(LuaCollisionEvent::Started {
                            b1: LuaRigidBodyHandle(rb1),
                            b2: LuaRigidBodyHandle(rb2),
                            sensor: flags.contains(CollisionEventFlags::SENSOR),
                            removed: flags.contains(CollisionEventFlags::REMOVED),
                        });
                    }
                }
            }
            CollisionEvent::Stopped(handle1, handle2, flags) => {
                if let (Some(rb1), Some(rb2)) = (
                    colliders.get(handle1).and_then(|col| col.parent()),
                    colliders.get(handle2).and_then(|col| col.parent()),
                ) {
                    if let Ok(ref mut data) = self.collision_event.lock() {
                        data.push(LuaCollisionEvent::Stopped {
                            b1: LuaRigidBodyHandle(rb1),
                            b2: LuaRigidBodyHandle(rb2),
                            sensor: flags.contains(CollisionEventFlags::SENSOR),
                            removed: flags.contains(CollisionEventFlags::REMOVED),
                        });
                    }
                }
            }
        }
    }
    fn handle_contact_force_event(
        &self,
        dt: f32,
        _bodies: &RigidBodySet,
        colliders: &ColliderSet,
        contact_pair: &ContactPair,
        total_force_magnitude: f32,
    ) {
        if let (Some(handle1), Some(handle2)) = (
            colliders
                .get(contact_pair.collider1)
                .and_then(|col| col.parent()),
            colliders
                .get(contact_pair.collider2)
                .and_then(|col| col.parent()),
        ) {
            if let Ok(ref mut data) = self.contact_force_event.lock() {
                data.push(LuaContactForceEvent {
                    h1: LuaRigidBodyHandle(handle1),
                    h2: LuaRigidBodyHandle(handle2),
                    dt,
                    total_force_magnitude,
                });
            }
        }
    }
}
