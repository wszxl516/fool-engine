// use super::LuaTextureHandle;
use crate::{
    graphics::types::LuaColor,
    lua::types::{LuaPoint, LuaSize},
    lua_table_get,
    resource::types::LuaTexture,
};
use egui::{Button, Rounding, Sense, Stroke};
use mlua::{FromLua, Function, LuaSerdeExt, Table, UserData, UserDataMethods, Value};
use nannou::glam::bool;
use nannou_egui::egui::{
    vec2, Align, Color32, ComboBox, Grid, Image, Layout, ProgressBar, Response, Slider, TextEdit,
    Ui, Widget,
};
use serde::{Deserialize, Serialize};
pub struct LuaUiContext<'a> {
    pub ui: &'a mut Ui,
}

pub struct LuaResponse {
    pub response: Response,
}

impl UserData for LuaResponse {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("clicked", |_, this, _: ()| Ok(this.response.clicked()));
        methods.add_method_mut("changed", |_, this, _: ()| Ok(this.response.changed()));
        methods.add_method_mut("double_clicked", |_, this, _: ()| {
            Ok(this.response.double_clicked())
        });
        methods.add_method_mut("middle_clicked", |_, this, _: ()| {
            Ok(this.response.middle_clicked())
        });
        methods.add_method_mut("secondary_clicked", |_, this, _: ()| {
            Ok(this.response.secondary_clicked())
        });
        methods.add_method_mut("hovered", |_, this, _: ()| Ok(this.response.hovered()));
        methods.add_method_mut("dragged", |_, this, _: ()| Ok(this.response.dragged()));
        methods.add_method_mut("has_focus", |_, this, _: ()| Ok(this.response.has_focus()));
        methods.add_method_mut("lost_focus", |_, this, _: ()| {
            Ok(this.response.lost_focus())
        });
        methods.add_method_mut("gained_focus", |_, this, _: ()| {
            Ok(this.response.gained_focus())
        });
        methods.add_method_mut("clicked_elsewhere", |_, this, _: ()| {
            Ok(this.response.clicked_elsewhere())
        });
    }
}

impl<'lua> UserData for LuaUiContext<'lua> {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("label", |lua, this, text: String| {
            let response = this.ui.label(text);
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut("button", |lua, this, label: String| {
            let response = this.ui.button(label);
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut("checkbox", |lua, this, table: Value| {
            let table = match table.as_table() {
                Some(table) => table,
                None => {
                    return Err(mlua::Error::RuntimeError(
                        "checkbox get userdata failed".to_owned(),
                    ))
                }
            };
            let mut checked = lua_table_get!(table, "checked", false);
            let label = lua_table_get!(table, "label", "".to_owned());
            let response = this.ui.checkbox(&mut checked, label);
            if response.changed() {
                table.set("checked", checked)?;
            }
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut("text_edit", |lua, this, text: Value| {
            let table = match text.as_table() {
                Some(table) => table,
                None => {
                    return Err(mlua::Error::RuntimeError(
                        "text_edit get userdata failed".to_owned(),
                    ))
                }
            };
            let mut content = lua_table_get!(table, "content", "".to_string());
            let text_edit = match lua_table_get!(table, "single_line", true) {
                true => TextEdit::singleline(&mut content),
                false => TextEdit::multiline(&mut content),
            };
            let text_edit = match lua_table_get!(table, "code_editor", false) {
                true => text_edit.code_editor(),
                false => text_edit,
            };
            let response = text_edit
                .cursor_at_end(true)
                .id_source(lua_table_get!(table, "id", "".to_string()))
                .char_limit(lua_table_get!(table, "char_limit", 256))
                .clip_text(lua_table_get!(table, "clip_text", false))
                .desired_rows(lua_table_get!(table, "rows", 1))
                .password(lua_table_get!(table, "password", false))
                .show(this.ui)
                .response;
            if response.changed() {
                table.set("content", content)?;
            }
            lua.create_userdata(LuaResponse { response })
        });
        methods.add_method_mut("slider", |lua, this, args: Value| {
            let table = match args.as_table() {
                Some(table) => table,
                None => {
                    return Err(mlua::Error::RuntimeError(
                        "text_edit get userdata failed".to_owned(),
                    ))
                }
            };
            let mut current = lua_table_get!(table, "current", 0usize);
            let min = lua_table_get!(table, "min", 0usize);
            let max = lua_table_get!(table, "max", 100usize);
            let label = lua_table_get!(table, "label", "".to_owned());
            let response = this
                .ui
                .add(Slider::new(&mut current, min..=max).text(label));
            if response.changed() {
                table.set("current", current)?;
            }
            Ok(lua.create_userdata(LuaResponse { response })?)
        });

        methods.add_method_mut("progress_bar", |lua, this, args: Value| {
            let table = match args.as_table() {
                Some(table) => table,
                None => {
                    return Err(mlua::Error::RuntimeError(
                        "text_edit get userdata failed".to_owned(),
                    ))
                }
            };
            let color = lua_table_get!(table, "color", LuaColor::default());
            let progress = lua_table_get!(table, "progress", 0f32);
            let show_percentage = lua_table_get!(table, "show_percentage", false);
            let name = lua_table_get!(table, "name", "".to_owned());
            let animate = lua_table_get!(table, "animate", false);

            let color = Color32::from_rgba_premultiplied(color.r, color.g, color.b, color.a);
            let progresspar = ProgressBar::new(progress)
                .animate(true)
                .text(name)
                .fill(color)
                .animate(animate);
            let progresspar = if show_percentage {
                progresspar.show_percentage()
            } else {
                progresspar
            };
            let response = progresspar.ui(this.ui);
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut("color_picker", |lua, this, args: Value| {
            let table = match args.as_table() {
                Some(table) => table,
                None => {
                    return Err(mlua::Error::RuntimeError(
                        "text_edit get userdata failed".to_owned(),
                    ))
                }
            };
            let r = lua_table_get!(table, "r", 0u8);
            let g = lua_table_get!(table, "g", 0u8);
            let b = lua_table_get!(table, "b", 0u8);
            let a = lua_table_get!(table, "a", 0u8);

            let mut color = Color32::from_rgba_premultiplied(r, g, b, a);
            let response = this.ui.color_edit_button_srgba(&mut color);
            if response.changed() {
                table.set("r", color.r())?;
                table.set("g", color.g())?;
                table.set("b", color.b())?;
                table.set("a", color.a())?;
            }
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut(
            "collapsing",
            |lua, this, (label, func): (String, Function)| {
                let lua_cloned = lua.clone();
                let response = this.ui.collapsing(label, move |ui| {
                    lua_cloned
                        .scope(|scope| {
                            let ctx = LuaUiContext { ui };
                            let ctx = scope.create_userdata(ctx)?;
                            func.call::<()>(ctx)?;
                            Ok(())
                        })
                        .unwrap();
                });
                lua.create_userdata(LuaResponse {
                    response: response.header_response,
                })
            },
        );

        methods.add_method_mut("horizontal", |lua, this, func: Function| {
            let lua_cloned = lua.clone();
            let response = this.ui.horizontal(move |ui| {
                let _ = lua_cloned.scope(|scope| {
                    let ctx = LuaUiContext { ui };
                    let ctx = scope.create_userdata(ctx)?;
                    func.call::<()>(ctx)?;
                    Ok(())
                });
            });
            lua.create_userdata(LuaResponse {
                response: response.response,
            })
        });

        methods.add_method_mut("vertical", |lua, this, func: Function| {
            let lua_cloned = lua.clone();
            let response = this.ui.vertical(move |ui| {
                lua_cloned
                    .scope(|scope| {
                        let ctx = LuaUiContext { ui };
                        let ctx = scope.create_userdata(ctx)?;
                        func.call::<()>(ctx)?;
                        Ok(())
                    })
                    .unwrap();
            });
            lua.create_userdata(LuaResponse {
                response: response.response,
            })
        });

        methods.add_method_mut("combo_box", |lua, this, args: Value| {
            let table = match args.as_table() {
                Some(table) => table,
                None => {
                    return Err(mlua::Error::RuntimeError(
                        "text_edit get userdata failed".to_owned(),
                    ))
                }
            };
            let id = lua_table_get!(table, "id", "".to_owned());
            let items = lua_table_get!(table, "items", Vec::<String>::new());
            let selected = lua_table_get!(
                table,
                "selected",
                items.get(0).unwrap_or(&"".to_owned()).to_owned()
            );
            let response = ComboBox::from_id_source(id)
                .selected_text(&selected)
                .show_ui(this.ui, move |ui| {
                    for item in &items {
                        let response = ui.selectable_label(selected == *item, item);
                        if response.clicked() {
                            table.set("selected", item.clone()).unwrap();
                        }
                    }
                });
            lua.create_userdata(LuaResponse {
                response: response.response,
            })
        });

        methods.add_method_mut(
            "grid",
            |lua, this, (id, spacing, start_row, func): (String, LuaSize<f32>, usize, Function)| {
                let lua_cloned = lua.clone();
                let response = Grid::new(id)
                    .spacing([spacing.w, spacing.h])
                    .start_row(start_row)
                    .show(this.ui, move |ui| {
                        lua_cloned
                            .scope(|scope| {
                                let ctx = LuaUiContext { ui };
                                let ctx = scope.create_userdata(ctx)?;
                                func.call::<()>(ctx)?;
                                Ok(())
                            })
                            .unwrap();
                    });
                lua.create_userdata(LuaResponse {
                    response: response.response,
                })
            },
        );

        methods.add_method_mut("separator", |lua, this, _: ()| {
            let response = this.ui.separator();
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut("heading", |lua, this, text: String| {
            let response = this.ui.heading(text);
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut("hyperlink", |lua, this, url: String| {
            let response = this.ui.hyperlink(url);
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut("small", |lua, this, text: String| {
            let response = this.ui.small(text);
            lua.create_userdata(LuaResponse { response })
        });

        methods.add_method_mut(
            "radio",
            |lua, this, (args, left_to_right): (Value, bool)| {
                let radios: Table = match args {
                    Value::Table(t) => t,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: args.type_name(),
                            to: "Table".to_owned(),
                            message: Some("expected table".to_string()),
                        });
                    }
                };
                let mut clicked_index = None;
                let algin = if left_to_right {
                    Layout::left_to_right(Align::Center)
                } else {
                    Layout::top_down(Align::Center)
                };
                let response = this.ui.with_layout(algin, |ui| {
                    for i in 1..=radios.len().unwrap_or(0) {
                        if let Ok(entry) = radios.get::<Table>(i) {
                            let selected = entry.get::<bool>("selected").unwrap_or(false);
                            let text = entry.get::<String>("text").unwrap_or_default();
                            let r = ui.radio(selected, &text);
                            if r.clicked() {
                                clicked_index = Some(i);
                            }
                        }
                    }
                });

                if let Some(clicked_i) = clicked_index {
                    for i in 1..=radios.len()? {
                        if let Ok(entry) = radios.get::<Table>(i) {
                            entry.set("selected", i == clicked_i)?;
                        }
                    }
                }

                lua.create_userdata(LuaResponse {
                    response: response.response,
                })
            },
        );
        methods.add_method_mut(
            "selectable_label",
            |lua, this, (selected, label): (bool, String)| {
                let response = this.ui.selectable_label(selected, label);
                lua.create_userdata(LuaResponse { response })
            },
        );

        methods.add_method_mut(
            "with_layout",
            |lua, this, (topdown_or_leftright, func): (bool, Function)| {
                let layout = if topdown_or_leftright {
                    Layout::top_down(Align::Center)
                        .with_cross_align(Align::Center)
                        .with_main_wrap(false)
                } else {
                    Layout::left_to_right(Align::Center)
                        .with_cross_align(Align::Center)
                        .with_main_wrap(false)
                };
                let lua = lua.clone();
                let response = this.ui.with_layout(layout, |ui| {
                    lua.scope(|scope| {
                        let ctx = scope.create_userdata(LuaUiContext { ui })?;
                        func.call::<()>(ctx)?;
                        Ok(())
                    })
                    .unwrap();
                });
                lua.create_userdata(LuaResponse {
                    response: response.response,
                })
            },
        );
        methods.add_method_mut("image", |lua, this, (texture, args): (Value, Value)| {
            let args: Table = match args {
                Value::Table(t) => t,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: args.type_name(),
                        to: "Table".to_owned(),
                        message: Some("expected table".to_string()),
                    });
                }
            };
            let texture = match texture {
                mlua::Value::UserData(ud) => ud.borrow::<LuaTexture>()?,
                _ => {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: texture.type_name(),
                        to: "EguiContext".into(),
                        message: Some("Expected EguiContext as UserData".into()),
                    });
                }
            };
            let show_loading_spinner = lua_table_get!(args, "show_loading_spinner", false);
            let rotate_origin = lua_table_get!(args, "rotate_origin", LuaPoint::<f32>::default());
            let bg_fill = lua_table_get!(args, "bg_fill", LuaColor::default());
            let rotate_angle = lua_table_get!(args, "rotate_angle", 0f32);
            let w = lua_table_get!(args, "w", 0f32);
            let h = lua_table_get!(args, "h", 0f32);
            let nw = lua_table_get!(args, "rounding_nw", 0f32);
            let ne = lua_table_get!(args, "rounding_ne", 0f32);
            let sw = lua_table_get!(args, "rounding_sw", 0f32);
            let se = lua_table_get!(args, "rounding_se", 0f32);

            let img = Image::from_texture(&texture.ui)
                .max_size(vec2(w, h))
                .rotate(rotate_angle, vec2(rotate_origin.x, rotate_origin.y))
                .rounding(Rounding { nw, ne, sw, se })
                .show_loading_spinner(show_loading_spinner)
                .bg_fill(Color32::from_rgba_premultiplied(
                    bg_fill.r, bg_fill.g, bg_fill.b, bg_fill.a,
                ))
                .texture_options(Default::default());
            let response = this.ui.add(img);
            lua.create_userdata(LuaResponse { response: response })
        });
        methods.add_method_mut(
            "image_button",
            |lua, this, (texture, args): (Value, Value)| {
                let args: Table = match args {
                    Value::Table(t) => t,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: args.type_name(),
                            to: "Table".to_owned(),
                            message: Some("expected table".to_string()),
                        });
                    }
                };
                let texture = match texture {
                    mlua::Value::UserData(ud) => ud.borrow::<LuaTexture>()?,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: texture.type_name(),
                            to: "EguiContext".into(),
                            message: Some("Expected EguiContext as UserData".into()),
                        });
                    }
                };
                let show_loading_spinner = lua_table_get!(args, "show_loading_spinner", false);
                let rotate_origin =
                    lua_table_get!(args, "rotate_origin", LuaPoint::<f32>::default());
                let bg_fill = lua_table_get!(args, "bg_fill", LuaColor::default());
                let rotate_angle = lua_table_get!(args, "rotate_angle", 0f32);
                let w = lua_table_get!(args, "w", 0f32);
                let h = lua_table_get!(args, "h", 0f32);
                let nw = lua_table_get!(args, "rounding_nw", 0f32);
                let ne = lua_table_get!(args, "rounding_ne", 0f32);
                let sw = lua_table_get!(args, "rounding_sw", 0f32);
                let se = lua_table_get!(args, "rounding_se", 0f32);
                let frame = lua_table_get!(args, "frame", false);

                let stroke_width = lua_table_get!(args, "stroke_width", 0f32);
                let stroke_color = lua_table_get!(args, "stroke_color", LuaColor::default());
                let wrap = lua_table_get!(args, "wrap", false);
                let bg_fill_color =
                    Color32::from_rgba_premultiplied(bg_fill.r, bg_fill.g, bg_fill.b, bg_fill.a);
                let img = Image::from_texture(&texture.ui)
                    .max_size(vec2(w, h))
                    .rotate(rotate_angle, vec2(rotate_origin.x, rotate_origin.y))
                    .rounding(Rounding { nw, ne, sw, se })
                    .show_loading_spinner(show_loading_spinner)
                    .bg_fill(bg_fill_color)
                    .texture_options(Default::default());
                let btn = Button::image(img)
                    .fill(bg_fill_color)
                    .rounding(Rounding { nw, ne, sw, se })
                    .stroke(Stroke::new(
                        stroke_width,
                        Color32::from_rgba_premultiplied(
                            stroke_color.r,
                            stroke_color.g,
                            stroke_color.b,
                            stroke_color.a,
                        ),
                    ))
                    .sense(Sense {
                        click: true,
                        drag: true,
                        focusable: true,
                    })
                    .wrap(wrap)
                    .frame(frame);
                let response = this.ui.add(btn);
                lua.create_userdata(LuaResponse { response: response })
            },
        );
        methods.add_method_mut("end_row", |_lua, this, (): ()| {
            this.ui.end_row();
            Ok(())
        });
        methods.add_method_mut("set_max_size", |_lua, this, size: LuaSize<f32>| {
            this.ui.set_max_size(vec2(size.w, size.h));
            Ok(())
        });
        methods.add_method_mut("set_min_size", |_lua, this, size: LuaSize<f32>| {
            this.ui.set_min_size(vec2(size.w, size.h));
            Ok(())
        });
        methods.add_method_mut("set_row_height", |_lua, this, height: f32| {
            this.ui.set_row_height(height);
            Ok(())
        });
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Radio {
    selected: bool,
    text: String,
}

impl FromLua for Radio {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
