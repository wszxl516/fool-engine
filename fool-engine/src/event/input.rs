use crate::script::types::LuaPoint;
use mlua::{
    Function, LuaSerdeExt, UserData, UserDataMethods,
    Value::{self},
};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use fool_window::WinEvent;
#[allow(dead_code)]
pub struct InputEvent<'a> {
    pub on_exit: Option<Function>,
    pub event: &'a WinEvent,
}

impl<'a> UserData for InputEvent<'a> {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("on_exit", |_lua, this, func: Function| {
            this.on_exit.replace(func);
            Ok(())
        });
        methods.add_method("key_pressed", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.event.key_pressed(key))
        });
        methods.add_method("key_released", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.event.key_released(key))
        });
        methods.add_method("key_held", |lua, this, key: Value| {
            let key: KeyCode = lua.from_value(key)?;
            Ok(this.event.key_held(key))
        });
        methods.add_method("get_mouse_position", |_, this, ()| {
            let diff = this.event.mouse_diff();
            Ok(LuaPoint {
                x: diff.0,
                y: diff.1,
            })
        });

        methods.add_method("mouse_pressed", |_, this, button: String| {
            let btn = match button.to_lowercase().as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => return Ok(false),
            };
            Ok(this.event.mouse_pressed(btn))
        });
        methods.add_method("mouse_released", |_, this, button: String| {
            let btn = match button.to_lowercase().as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => return Ok(false),
            };
            Ok(this.event.mouse_released(btn))
        });
        // methods.add_method("mouse_wheel", |lua, this, ()| {
        //     this.event.
        //     let table = lua.create_table()?;
        //     match this.mouse_wheel.0 {
        //         None => {
        //             table.set("delta", Nil)?;
        //         }
        //         Some(delta) => match delta {
        //             MouseScrollDelta::LineDelta(x, y) => {
        //                 let delta_table = lua.create_table()?;
        //                 delta_table.set("line", LuaPoint { x: x, y: y })?;
        //                 table.set("delta", delta_table)?;
        //             }
        //             MouseScrollDelta::PixelDelta(p) => {
        //                 let pixel_table = lua.create_table()?;
        //                 pixel_table.set("pixel", LuaPoint { x: p.x, y: p.y })?;
        //                 table.set("delta", pixel_table)?;
        //             }
        //         },
        //     }
        //     match this.mouse_wheel.1 {
        //         None => {
        //             table.set("touch", Nil)?;
        //         }
        //         Some(touch) => match touch {
        //             TouchPhase::Started => {
        //                 table.set("touch", "Started")?;
        //             }
        //             TouchPhase::Moved => {
        //                 table.set("touch", "Moved")?;
        //             }
        //             TouchPhase::Ended => {
        //                 table.set("touch", "Ended")?;
        //             }
        //             TouchPhase::Cancelled => {
        //                 table.set("touch", "Cancelled")?;
        //             }
        //         },
        //     }
        //     Ok(table)
        // });

        // methods.add_method("mouse_entered", |_lua, this, ()| Ok(this.event));
        // methods.add_method("focused", |_lua, this, ()| Ok(this.event));
        // methods.add_method("ime_state", |lua, this, ()| {
        //     let table = lua.create_table()?;
        //     match &this.ime {
        //         Ime::Enabled => {
        //             table.set("state", "enabled")?;
        //         }
        //         Ime::Disabled => {
        //             table.set("state", "disabled")?;
        //         }
        //         Ime::Preedit(s, pos) => {
        //             table.set("state", "preedit")?;
        //             let preedit = lua.create_table()?;
        //             preedit.set("content", s.clone())?;
        //             match pos {
        //                 Some(p) => preedit.set("pos", LuaPoint { x: p.0, y: p.1 })?,
        //                 None => preedit.set("pos", Nil)?,
        //             }
        //             table.set("preedit", preedit)?;
        //         }
        //         Ime::Commit(s) => {
        //             table.set("state", "commit")?;
        //             table.set("commit", s.clone())?;
        //         }
        //     }
        //     Ok(table)
        // });
    }
}
