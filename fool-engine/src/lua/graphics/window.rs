use super::super::gui::{create_window, LuaUIConfig};
use super::types::{LuaPoint, LuaSize};
use crate::event::EngineEventLoop;
use crate::map2lua_error;
use crate::resource::ResourceManager;
use mlua::{Function, UserData, UserDataMethods, Value};
use parking_lot::Mutex;
use std::{str::FromStr, sync::Arc};
use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position, Size},
    window::{CursorGrabMode, CursorIcon, Fullscreen, Window},
};

pub struct LuaWindow {
    pub window: Arc<Window>,
    pub resource: Arc<Mutex<ResourceManager>>,
    pub event_loop: EngineEventLoop,
}
impl UserData for LuaWindow {
    //cursor
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("exit", |_lua, this, ()| {
            this.event_loop.exit_window();
            Ok(())
        });
        methods.add_method("set_cursor_grab", |_lua, this, enable: String| {
            let grab = match enable.as_str() {
                "None" => CursorGrabMode::None,
                "Confined" => CursorGrabMode::Confined,
                "Locked" => CursorGrabMode::Locked,
                _ => CursorGrabMode::None,
            };
            map2lua_error!(this.window.set_cursor_grab(grab), "set_cursor_grab")
        });
        methods.add_method("set_ime_allowed", |_lua, this, enable: bool| {
            this.window.set_ime_allowed(enable);
            Ok(())
        });
        methods.add_method(
            "set_ime_cursor_area",
            |_lua, this, (pos, size): (LuaPoint<f64>, LuaSize<f64>)| {
                this.window.set_ime_cursor_area(
                    Position::Logical(LogicalPosition::new(pos.x, pos.y)),
                    Size::Logical(LogicalSize::new(size.w, size.h)),
                );
                Ok(())
            },
        );
        methods.add_method("load_cursor_icon", |_lua, this, cursor: String| {
            this.event_loop.load_cursor(cursor);
            Ok(())
        });
        methods.add_method("set_cursor_icon", |_lua, this, cursor: String| {
            if let Some(cursor) = this.resource.lock().get_cursor(&cursor) {
                this.window.set_cursor(cursor.clone());
            }
            Ok(())
        });
        methods.add_method("set_window_icon", |_lua, this, icon: String| {
            match this.resource.lock().get_window_icon(&icon) {
                Ok(icon) => this.window.set_window_icon(Some(icon.clone())),
                Err(err) => log::error!("failed to get window icon {}, {}", icon, err),
            }
            Ok(())
        });
        methods.add_method("set_cursor", |_lua, this, cursor: String| {
            if let Ok(cursor) = CursorIcon::from_str(&cursor) {
                this.window.set_cursor(cursor);
            }
            Ok(())
        });
        methods.add_method("set_cursor_visible", |_lua, this, visible: bool| {
            this.window.set_cursor_visible(visible);
            Ok(())
        });
        methods.add_method("set_fullscreen", |_lua, this, visible: bool| {
            let c = if visible {
                Some(Fullscreen::Borderless(this.window.current_monitor()))
            } else {
                None
            };
            this.window.set_fullscreen(c);
            Ok(())
        });
        methods.add_method("set_max_inner_size", |_lua, this, size: LuaSize<f64>| {
            this.window
                .set_max_inner_size(Some(PhysicalSize::new(size.w, size.h)));
            Ok(())
        });
        methods.add_method("set_min_inner_size", |_lua, this, size: LuaSize<f64>| {
            this.window
                .set_min_inner_size(Some(PhysicalSize::new(size.w, size.h)));
            Ok(())
        });

        methods.add_method("set_maximized", |_lua, this, maximized: bool| {
            this.window.set_maximized(maximized);
            Ok(())
        });
        methods.add_method("set_minimized", |_lua, this, minimized: bool| {
            this.window.set_minimized(minimized);
            Ok(())
        });
        methods.add_method("set_decorations", |_lua, this, decorations: bool| {
            this.window.set_decorations(decorations);
            Ok(())
        });

        methods.add_method("set_resizable", |_lua, this, resizable: bool| {
            this.window.set_resizable(resizable);
            Ok(())
        });

        methods.add_method("set_title", |_lua, this, title: String| {
            this.window.set_title(&title);
            Ok(())
        });
        methods.add_method("set_visible", |_lua, this, visible: bool| {
            this.window.set_visible(visible);
            Ok(())
        });
        methods.add_method("inner_size", |_lua, this, (): ()| {
            let size = this.window.inner_size();
            Ok(LuaSize {
                w: size.width,
                h: size.height,
            })
        });
        methods.add_method("outer_size", |_lua, this, (): ()| {
            let size = this.window.outer_size();
            Ok(LuaSize {
                w: size.width,
                h: size.height,
            })
        });

        methods.add_method("monitor", |lua, this, ()| {
            let monitor = this.window.current_monitor();
            match monitor {
                None => Ok(None),
                Some(m) => {
                    let info = lua.create_table()?;
                    info.set("name", m.name())?;
                    let position = m.position();
                    let pos_table = lua.create_table()?;
                    pos_table.set("x", position.x)?;
                    pos_table.set("y", position.y)?;
                    info.set("position", pos_table)?;
                    info.set("refresh_rate_millihertz", m.refresh_rate_millihertz())?;
                    info.set("scale_factor", m.scale_factor())?;
                    let size = m.size();
                    let size_table = lua.create_table()?;
                    size_table.set("w", size.width)?;
                    size_table.set("h", size.height)?;
                    info.set("size", size_table)?;
                    Ok(Some(info))
                }
            }
        });
        methods.add_method(
            "gui_window",
            |lua, _, (config, context, func): (LuaUIConfig, Value, Function)| {
                create_window(lua, config, context, func)
            },
        );
    }
}
