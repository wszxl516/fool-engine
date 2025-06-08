use crate::script::types::LuaPoint;
use mlua::{
    LuaSerdeExt, UserData, UserDataMethods,
    Value::{self},
};
use winit::{
    event::{Ime, MouseButton},
    keyboard::{Key, KeyCode},
};

use fool_window::WinEvent;
#[allow(dead_code)]
pub struct InputEvent<'a> {
    pub events: &'a Vec<WinEvent>,
}

impl<'a> InputEvent<'a> {
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        for event in self.events {
            if event.key_pressed(key) {
                return true;
            }
        }
        false
    }
    pub fn key_released(&self, key: KeyCode) -> bool {
        for event in self.events {
            if event.key_released(key) {
                return true;
            }
        }
        false
    }
    pub fn key_held(&self, key: KeyCode) -> bool {
        for event in self.events {
            if event.key_held(key) {
                return true;
            }
        }
        false
    }
    pub fn cursor(&self) -> (f32, f32) {
        let mut x = -1f32;
        let mut y = -1f32;
        for event in self.events {
            if let Some(pos) = event.cursor() {
                x += pos.0;
                y += pos.1;
            }
        }
        (x, y)
    }

    pub fn cursor_diff(&self) -> (f32, f32) {
        let mut x = -1f32;
        let mut y = -1f32;
        for event in self.events {
            let pos = event.cursor_diff();
            x += pos.0;
            y += pos.1;
        }
        (x, y)
    }
    pub fn mouse_pressed(&self, key: MouseButton) -> bool {
        for event in self.events {
            if event.mouse_pressed(key) {
                return true;
            }
        }
        false
    }
    pub fn mouse_released(&self, key: MouseButton) -> bool {
        for event in self.events {
            if event.mouse_released(key) {
                return true;
            }
        }
        false
    }
    pub fn scroll_diff(&self) -> (f32, f32) {
        if let Some(e) = self.events.last() {
            e.scroll_diff()
        } else {
            (0.0, 0.0)
        }
    }
    pub fn is_cursor_active(&self) -> bool {
        if let Some(e) = self.events.last() {
            e.is_cursor_active()
        } else {
            false
        }
    }
    pub fn focused(&self) -> bool {
        if let Some(e) = self.events.last() {
            e.focused()
        } else {
            false
        }
    }
    pub fn raw_keys(&self) -> Vec<KeyRepr> {
        let mut all_keys = Vec::new();
        for event in self.events {
            let keys = KeyRepr::from_keys(event.text());
            all_keys.extend(keys);
        }
        all_keys
    }
    pub fn ime(&self) -> Option<Ime> {
        self.events
            .iter()
            .rev()
            .find_map(|e| match e.ime() {
                Some(Ime::Commit(_)) => Some(e.clone()),
                Some(Ime::Preedit(_, _)) => Some(e.clone()),
                Some(Ime::Enabled) | Some(Ime::Disabled) => Some(e.clone()),
                None => None,
            })
            .and_then(|e| e.ime())
    }
}

impl<'a> UserData for InputEvent<'a> {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("key_pressed", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.key_pressed(key))
        });
        methods.add_method("key_released", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.key_released(key))
        });
        methods.add_method("key_held", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.key_held(key))
        });
        methods.add_method("cursor_pos", |lua, this, ()| {
            let table = lua.create_table()?;
            let diff = this.cursor();
            table.set("x", diff.0)?;
            table.set("y", diff.1)?;
            Ok(Value::Table(table))
        });
        methods.add_method("cursor_diff", |lua, this, ()| {
            let diff = this.cursor_diff();
            let table = lua.create_table()?;
            table.set("x", diff.0)?;
            table.set("y", diff.1)?;
            Ok(Value::Table(table))
        });

        methods.add_method("mouse_pressed", |_, this, button: String| {
            let btn = match button.to_lowercase().as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => return Ok(false),
            };
            Ok(this.mouse_pressed(btn))
        });
        methods.add_method("mouse_released", |_, this, button: String| {
            let btn = match button.to_lowercase().as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => return Ok(false),
            };
            Ok(this.mouse_released(btn))
        });
        methods.add_method("scroll_diff", |lua, this, ()| {
            let diff = this.scroll_diff();
            let table = lua.create_table()?;
            table.set("x", diff.0)?;
            table.set("y", diff.1)?;
            Ok(Value::Table(table))
        });

        methods.add_method(
            "cursor_active",
            |_lua, this, ()| Ok(this.is_cursor_active()),
        );
        methods.add_method("raw_keys", |_lua, this, ()| Ok(this.raw_keys()));
        methods.add_method("focused", |_lua, this, ()| Ok(this.focused()));
        methods.add_method("ime_state", |lua, this, ()| {
            let table = lua.create_table()?;
            if let Some(ime) = this.ime() {
                match &ime {
                    Ime::Enabled => {
                        table.set("state", "enabled")?;
                    }
                    Ime::Disabled => {
                        table.set("state", "disabled")?;
                    }
                    Ime::Preedit(s, pos) => {
                        table.set("state", "preedit")?;
                        let preedit = lua.create_table()?;
                        preedit.set("content", s.clone())?;
                        match pos {
                            Some(p) => preedit.set("pos", LuaPoint { x: p.0, y: p.1 })?,
                            None => preedit.set("pos", Value::Nil)?,
                        }
                        table.set("preedit", preedit)?;
                    }
                    Ime::Commit(s) => {
                        table.set("state", "commit")?;
                        table.set("commit", s.clone())?;
                    }
                }
                Ok(Value::Table(table))
            } else {
                Ok(Value::Nil)
            }
        });
    }
}

#[derive(Clone)]
pub enum KeyRepr {
    Named(String),
    Character(String),
}
impl KeyRepr {
    pub fn from_keys(keys: &[Key]) -> Vec<KeyRepr> {
        let mut keyrepr = Vec::new();
        for key in keys {
            match key {
                Key::Named(named) => keyrepr.push(KeyRepr::Named(format!("{:?}", named))),
                Key::Character(s) => keyrepr.push(KeyRepr::Character(s.to_string())),
                _ => continue,
            }
        }
        keyrepr
    }
}

impl UserData for KeyRepr {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("kind", |_, this| {
            let kind = match this {
                KeyRepr::Named(_) => "mamed",
                KeyRepr::Character(_) => "character",
            };
            Ok(kind)
        });

        fields.add_field_method_get("value", |_, this| {
            let val = match this {
                KeyRepr::Named(v) => v.clone(),
                KeyRepr::Character(v) => v.clone(),
            };
            Ok(val)
        });
    }
}
