pub use super::super::gui::EguiContext;
use super::types::{LuaGuiStyle, LuaPoint, LuaSize};
use crate::{map2lua_error, resource::types::LuaImage};
use egui::{Style, Visuals};
use mlua::{UserData, Value};
use winit::window::Icon;
use winit::window::Window;
pub struct LuaWindow<'a> {
    pub window: std::cell::Ref<'a, Window>,
}
impl UserData for LuaWindow<'_> {
    //cursor
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("set_ime_allowed", |_lua, this, enable: bool| {
            this.window.set_ime_allowed(enable);
            Ok(())
        });

        methods.add_method("set_cursor_visible", |_lua, this, visible: bool| {
            this.window.set_cursor_visible(visible);
            Ok(())
        });
        //window

        methods.add_method("set_decorations", |_lua, this, decorations: bool| {
            this.window.set_decorations(decorations);
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
            let icon = map2lua_error!(
                Icon::from_rgba(image.into_raw(), size.0, size.1),
                "set_window_icon create icon Error: {}"
            )?;
            this.window.set_window_icon(Some(icon));
            Ok(())
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
                style.wrap_mode = ui_style.wrap;
                context.context.set_style(style);
                Ok(())
            },
        );
    }
}
