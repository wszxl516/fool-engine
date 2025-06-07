use crate::event::EngineEventLoop;
use crate::script::types::LuaPoint;
use mlua::{
    Function, LuaSerdeExt, UserData, UserDataMethods,
    Value::{self, Nil},
};
use std::collections::HashSet;
use winit::event::{Ime, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Clone)]
pub struct EventState {
    pub keys_pressed: HashSet<KeyCode>,
    pub keys_released: HashSet<KeyCode>,
    pub keys_held: HashSet<KeyCode>,
    pub mouse_position: (f32, f32),
    pub mouse_buttons_pressed: HashSet<MouseButton>,
    pub mouse_buttons_released: HashSet<MouseButton>,
    pub mouse_wheel: (Option<MouseScrollDelta>, Option<TouchPhase>),
    pub ime: Ime,
    pub mouse_entered: bool,
    pub focused: bool,
    on_exit: Option<Function>,
    pub event_loop: EngineEventLoop,
}
impl EventState {
    pub fn new(event_loop: EngineEventLoop) -> Self {
        Self {
            keys_pressed: Default::default(),
            keys_released: Default::default(),
            keys_held: Default::default(),
            mouse_position: Default::default(),
            mouse_buttons_pressed: Default::default(),
            mouse_buttons_released: Default::default(),
            mouse_wheel: (None, None),
            mouse_entered: false,
            focused: false,
            ime: Ime::Disabled,
            on_exit: None,
            event_loop,
        }
    }
}
impl EventState {
    fn begin_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_released.clear();
        self.mouse_wheel = (None, None);
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.begin_frame();
        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                is_synthetic: _,
                event,
            } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    match event.state {
                        winit::event::ElementState::Pressed => {
                            if self.keys_held.insert(key) {
                                self.keys_pressed.insert(key);
                            }
                        }
                        winit::event::ElementState::Released => {
                            if self.keys_held.remove(&key) {
                                self.keys_released.insert(key);
                            }
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                winit::event::ElementState::Pressed => {
                    self.mouse_buttons_pressed.insert(*button);
                }
                winit::event::ElementState::Released => {
                    self.mouse_buttons_released.insert(*button);
                    self.mouse_buttons_pressed.remove(button);
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x as f32, position.y as f32);
            }
            WindowEvent::MouseWheel { delta, phase, .. } => {
                self.mouse_wheel = (Some(*delta), Some(*phase));
            }
            WindowEvent::CursorEntered { .. } => {
                self.mouse_entered = true;
            }
            WindowEvent::CursorLeft { .. } => {
                self.mouse_entered = false;
            }
            WindowEvent::Focused(focused) => {
                self.focused = *focused;
            }
            WindowEvent::Ime(ime) => {
                self.ime = ime.clone();
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                if let Some(func) = &self.on_exit {
                    log::debug!("exit call from lua");
                    let _ = func.call::<()>(());
                    self.event_loop.exit_window();
                }
            }
            _ => {}
        }
    }
}

impl UserData for &mut EventState {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("on_exit", |_lua, this, func: Function| {
            this.on_exit.replace(func);
            Ok(())
        });
        methods.add_method("key_pressed", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.keys_pressed.contains(&key))
        });
        methods.add_method("key_released", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.keys_released.contains(&key))
        });
        methods.add_method("key_held", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.keys_held.contains(&key))
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

        methods.add_method("mouse_entered", |_lua, this, ()| Ok(this.mouse_entered));
        methods.add_method("focused", |_lua, this, ()| Ok(this.focused));
        methods.add_method("ime_state", |lua, this, ()| {
            let table = lua.create_table()?;
            match &this.ime {
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
                        None => preedit.set("pos", Nil)?,
                    }
                    table.set("preedit", preedit)?;
                }
                Ime::Commit(s) => {
                    table.set("state", "commit")?;
                    table.set("commit", s.clone())?;
                }
            }
            Ok(table)
        });
    }
}
