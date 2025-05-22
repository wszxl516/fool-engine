pub use super::super::gui::EguiContext;
use super::types::{LuaGuiStyle, LuaPoint, LuaSize};
use crate::{
    map2lua_error,
    resource::types::{LuaFont, LuaImage},
};
use mlua::{Error as LuaError, FromLua, LuaSerdeExt, UserData, Value};
use nannou::window::Window;
use nannou_egui::egui::{Style, Visuals};
use serde::{Deserialize, Serialize};
use winit::window::{CursorIcon, Icon};
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct LuaCursorIcon(CursorIcon);
impl FromLua for LuaCursorIcon {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
pub struct LuaWindow<'a> {
    pub window: std::cell::Ref<'a, Window>,
}
impl UserData for LuaWindow<'_> {
    //cursor
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("set_cursor_grab", |_lua, this, rgba: bool| {
            map2lua_error!(
                this.window.set_cursor_grab(rgba),
                "window set_cursor_grab failed!"
            );
            Ok(())
        });
        methods.add_method("set_cursor_icon", |_lua, this, icon: LuaCursorIcon| {
            this.window.set_cursor_icon(icon.0);
            Ok(())
        });

        methods.add_method(
            "set_cursor_position_points",
            |_lua, this, position: LuaPoint<f32>| {
                map2lua_error!(
                    this.window
                        .set_cursor_position_points(position.x, position.y),
                    "window set_cursor_position_points failed!"
                );
                Ok(())
            },
        );

        methods.add_method("set_cursor_visible", |_lua, this, visible: bool| {
            this.window.set_cursor_visible(visible);
            Ok(())
        });
        //window
        methods.add_method("set_always_on_top", |_lua, this, always_on_top: bool| {
            this.window.set_always_on_top(always_on_top);
            Ok(())
        });
        methods.add_method("set_decorations", |_lua, this, decorations: bool| {
            this.window.set_decorations(decorations);
            Ok(())
        });

        methods.add_method("set_fullscreen", |_lua, this, fullscreen: bool| {
            this.window.set_fullscreen(fullscreen);
            Ok(())
        });
        methods.add_method(
            "set_ime_position_points",
            |_lua, this, position: LuaPoint<f32>| {
                this.window.set_ime_position_points(position.x, position.y);
                Ok(())
            },
        );
        methods.add_method("set_inner_size_pixels", |_lua, this, (w, h): (u32, u32)| {
            this.window.set_inner_size_pixels(w, h);
            Ok(())
        });
        methods.add_method("set_inner_size_points", |_lua, this, (w, h): (f32, f32)| {
            this.window.set_inner_size_points(w, h);
            Ok(())
        });
        methods.add_method(
            "set_max_inner_size_points",
            |_lua, this, size: Option<LuaSize<f32>>| {
                this.window
                    .set_max_inner_size_points(size.map(|s| (s.w, s.h)));
                Ok(())
            },
        );
        methods.add_method("set_maximized", |_lua, this, maximized: bool| {
            this.window.set_maximized(maximized);
            Ok(())
        });
        methods.add_method(
            "set_min_inner_size_points",
            |_lua, this, size: Option<LuaSize<f32>>| {
                this.window
                    .set_min_inner_size_points(size.map(|s| (s.w, s.h)));
                Ok(())
            },
        );
        methods.add_method("set_minimized", |_lua, this, minimized: bool| {
            this.window.set_minimized(minimized);
            Ok(())
        });
        methods.add_method(
            "set_outer_position_pixels",
            |_lua, this, position: LuaPoint<i32>| {
                this.window
                    .set_outer_position_pixels(position.x, position.y);
                Ok(())
            },
        );

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
        methods.add_method("set_window_icon", |_lua, this, image: Value| {
            let img = match image {
                mlua::Value::UserData(ud) => ud.borrow::<LuaImage>()?,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: image.type_name(),
                        to: "EguiContext".into(),
                        message: Some("Expected EguiContext as UserData".into()),
                    });
                }
            };
            let image = img.image.to_rgba8();
            let size = image.dimensions();
            let icon = Icon::from_rgba(image.into_raw(), size.0, size.1).map_err(|e| {
                LuaError::RuntimeError(format!("set_window_icon create icon Error: {}", e))
            })?;

            this.window.set_window_icon(Some(icon));
            Ok(())
        });
        methods.add_method("inner_size_pixels", |_lua, this, ()| {
            let size = this.window.inner_size_pixels();
            Ok(LuaSize {
                w: size.0,
                h: size.1,
            })
        });
        methods.add_method("outer_size_pixels", |_lua, this, ()| {
            let size = this.window.outer_size_pixels();
            Ok(LuaSize {
                w: size.0,
                h: size.1,
            })
        });

        methods.add_method("inner_size_points", |_lua, this, ()| {
            let size = this.window.inner_size_points();
            Ok(LuaSize {
                w: size.0,
                h: size.1,
            })
        });
        methods.add_method("outer_size_points", |_lua, this, ()| {
            let size = this.window.outer_size_points();
            Ok(LuaSize {
                w: size.0,
                h: size.1,
            })
        });
        methods.add_method("elapsed_frames", |_lua, this, ()| {
            Ok(this.window.elapsed_frames())
        });
        methods.add_method("capture_frame", |_lua, this, path: String| {
            this.window.capture_frame(path);
            Ok(())
        });
        methods.add_method("msaa_samples", |_lua, this, ()| {
            Ok(this.window.msaa_samples())
        });
        methods.add_method("rect", |_lua, this, ()| {
            let rect = this.window.rect();
            Ok([
                LuaPoint {
                    x: rect.x.start,
                    y: rect.y.start,
                },
                LuaPoint {
                    x: rect.x.end,
                    y: rect.y.end,
                },
            ])
        });
        methods.add_method("is_fullscreen", |_lua, this, ()| {
            let is_fullscreen = this.window.is_fullscreen();
            Ok(is_fullscreen)
        });
        methods.add_method("monitor", |lua, this, ()| {
            let monitor = this.window.current_monitor();
            match monitor {
                None => Ok(None),
                Some(m) => {
                    let info = lua.create_table()?;
                    info.set("name", m.name())?;
                    let position = m.position();
                    info.set(
                        "position",
                        LuaPoint {
                            x: position.x,
                            y: position.y,
                        },
                    )?;
                    info.set("refresh_rate_millihertz", m.refresh_rate_millihertz())?;
                    info.set("scale_factor", m.scale_factor())?;
                    let size = m.size();
                    info.set(
                        "size",
                        LuaSize {
                            w: size.width,
                            h: size.height,
                        },
                    )?;
                    Ok(Some(info))
                }
            }
        });
        methods.add_method(
            "set_gui_font",
            |_lua, _this, (context, font): (mlua::Value, mlua::Value)| {
                let context = match context {
                    mlua::Value::UserData(ud) => ud.borrow::<EguiContext>()?,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: context.type_name(),
                            to: "EguiContext".into(),
                            message: Some("Expected EguiContext as UserData".into()),
                        });
                    }
                };

                let font = match font {
                    mlua::Value::UserData(ud) => ud.borrow::<LuaFont>()?,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: font.type_name(),
                            to: "LuaFont".into(),
                            message: Some("Expected LuaFont as UserData".into()),
                        });
                    }
                };
                context.context.set_fonts(font.ui.clone());
                Ok(())
            },
        );

        methods.add_method(
            "set_gui_style",
            |_lua, _this, (context, ui_style): (mlua::Value, LuaGuiStyle)| {
                let context = match context {
                    mlua::Value::UserData(ud) => ud.borrow::<EguiContext>()?,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: context.type_name(),
                            to: "EguiContext".into(),
                            message: Some("Expected EguiContext as UserData".into()),
                        });
                    }
                };
                let mut style: Style = (*context.context.style()).clone();
                style.text_styles = ui_style.text_style();
                style.visuals = if ui_style.dark {
                    Visuals::dark()
                } else {
                    Visuals::light()
                };
                if let Some(color) = ui_style.noninteractive_fg_color {
                    style.visuals.widgets.noninteractive.fg_stroke.color = color.into();
                }
                if let Some(color) = ui_style.hovered_fg_color {
                    style.visuals.widgets.hovered.fg_stroke.color = color.into();
                }
                if let Some(color) = ui_style.active_fg_color {
                    style.visuals.widgets.active.fg_stroke.color = color.into();
                }
                if let Some(color) = ui_style.inactive_fg_color {
                    style.visuals.widgets.inactive.fg_stroke.color = color.into();
                }
                if let Some(color) = ui_style.open_fg_color {
                    style.visuals.widgets.open.fg_stroke.color = color.into();
                }
                style.animation_time = ui_style.animation_time;
                style.wrap = ui_style.wrap;
                context.context.set_style(style);
                Ok(())
            },
        );
    }
}
