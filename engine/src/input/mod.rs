use mlua::{
    LuaSerdeExt, UserData, UserDataMethods,
    Value::{self, Nil},
};
use nannou::event::WindowEvent;
use std::collections::HashSet;
use winit::event::{MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode};

use crate::lua::types::LuaPoint;
pub struct InputState {
    pub key_pressed: HashSet<VirtualKeyCode>,
    pub keys_released: HashSet<VirtualKeyCode>,
    pub mouse_position: (f32, f32),
    pub mouse_buttons_pressed: HashSet<MouseButton>,
    pub mouse_buttons_released: HashSet<MouseButton>,
    pub mouse_wheel: (Option<MouseScrollDelta>, Option<TouchPhase>),
    pub char: Option<char>,
    pub mouse_entered: bool,
    pub focused: bool,
}
impl Default for InputState {
    fn default() -> Self {
        Self {
            key_pressed: Default::default(),
            keys_released: Default::default(),
            mouse_position: Default::default(),
            mouse_buttons_pressed: Default::default(),
            mouse_buttons_released: Default::default(),
            char: None,
            mouse_wheel: (None, None),
            mouse_entered: false,
            focused: false,
        }
    }
}
impl InputState {
    pub fn begin_frame(&mut self) {
        self.keys_released.clear();
        self.key_pressed.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
        self.mouse_wheel = (None, None);
        self.char = None;
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyPressed(key) => {
                self.key_pressed.insert(*key);
            }
            WindowEvent::KeyReleased(key) => {
                self.keys_released.insert(*key);
            }
            WindowEvent::MouseMoved(position) => {
                self.mouse_position = (position.x, position.y);
            }
            WindowEvent::MousePressed(btn) => {
                self.mouse_buttons_pressed.insert(*btn);
            }
            WindowEvent::MouseReleased(btn) => {
                self.mouse_buttons_released.insert(*btn);
            }
            WindowEvent::MouseWheel(delta, t) => {
                self.mouse_wheel = (Some(delta.to_owned()), Some(t.to_owned()))
            }
            WindowEvent::ReceivedCharacter(c) => self.char = Some(*c),
            WindowEvent::MouseEntered => self.mouse_entered = true,
            WindowEvent::MouseExited => self.mouse_entered = false,
            WindowEvent::Focused => self.focused = true,
            WindowEvent::Unfocused => self.focused = false,
            _ => {}
        }
    }
}

impl UserData for &InputState {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("key_pressed", |lua, this, key: Value| {
            let key: VirtualKeyCode = lua.from_value(key)?;
            Ok(this.key_pressed.contains(&key))
        });

        methods.add_method("key_released", |lua, this, key: Value| {
            let key: VirtualKeyCode = lua.from_value(key)?;
            Ok(this.keys_released.contains(&key))
        });

        methods.add_method("get_mouse_position", |_, this, ()| {
            Ok(LuaPoint {
                x: this.mouse_position.0,
                y: this.mouse_position.1,
            })
        });

        methods.add_method("mouse_pressed", |_, this, button: String| {
            let btn = match button.to_lowercase().as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => return Ok(false),
            };
            Ok(this.mouse_buttons_pressed.contains(&btn))
        });
        methods.add_method("mouse_released", |_, this, button: String| {
            let btn = match button.to_lowercase().as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => return Ok(false),
            };
            Ok(this.mouse_buttons_released.contains(&btn))
        });
        methods.add_method("mouse_wheel", |lua, this, ()| {
            let table = lua.create_table()?;
            match this.mouse_wheel.0 {
                None => {
                    table.set("delta", Nil)?;
                }
                Some(delta) => match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        let delta_table = lua.create_table()?;
                        delta_table.set("line", LuaPoint { x: x, y: y })?;
                        table.set("delta", delta_table)?;
                    }
                    MouseScrollDelta::PixelDelta(p) => {
                        let pixel_table = lua.create_table()?;
                        pixel_table.set("pixel", LuaPoint { x: p.x, y: p.y })?;
                        table.set("delta", pixel_table)?;
                    }
                },
            }
            match this.mouse_wheel.1 {
                None => {
                    table.set("touch", Nil)?;
                }
                Some(touch) => match touch {
                    TouchPhase::Started => {
                        table.set("touch", "Started")?;
                    }
                    TouchPhase::Moved => {
                        table.set("touch", "Moved")?;
                    }
                    TouchPhase::Ended => {
                        table.set("touch", "Ended")?;
                    }
                    TouchPhase::Cancelled => {
                        table.set("touch", "Cancelled")?;
                    }
                },
            }
            Ok(table)
        });

        methods.add_method("get_char", |lua, this, ()| match this.char {
            None => Ok(Nil),
            Some(char) => Ok(lua.to_value(&char)?),
        });
        methods.add_method("mouse_entered", |_lua, this, ()| Ok(this.mouse_entered));
        methods.add_method("focused", |_lua, this, ()| Ok(this.focused));
    }
}
