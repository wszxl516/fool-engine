pub use super::graphics::types::LuaColor;
use mlua::{Function, Lua, UserData, Value};
pub mod binding;
pub mod types;
use crate::resource::ResourceManager;
pub use binding::LuaUiContext;
use egui::{pos2, vec2, Context, Visuals};
use parking_lot::Mutex;
use std::sync::Arc;
pub use types::{LuaGuiStyle, LuaUIConfig};
pub struct EguiContext {
    pub context: Context,
    pub width: f32,
    pub heigth: f32,
    pub resource: Arc<Mutex<ResourceManager>>,
}
impl UserData for EguiContext {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_font", |_lua, this, name: String| {
            let mut res = this.resource.lock();
            res.load_ui_font(name).unwrap();
            let font = res.ui_font.clone();
            this.context.set_fonts(font);
            Ok(())
        });
        methods.add_method("set_style", |_lua, this, ui_style: LuaGuiStyle| {
            let context = &this.context;
            let mut style = context.style().as_ref().clone();
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
            context.set_style(style);
            Ok(())
        });
    }
}

pub fn create_window(
    lua: &Lua,
    config: LuaUIConfig,
    context: Value,
    func: Function,
) -> mlua::Result<()> {
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
    let x_c = context.width / 2.0 + config.x - config.w / 2.0;
    let y_c = context.heigth / 2.0 - config.y - config.h / 2.0;
    let pos = pos2(x_c, y_c);
    let size = vec2(config.w, config.h);
    let resource = context.resource.clone();
    let texture = config.bg_img;
    egui::containers::Window::new(config.title)
        .collapsible(config.collapsible)
        .constrain(config.constrain)
        .default_open(config.default_open)
        .default_pos(pos)
        .default_size(size)
        .enabled(true)
        .drag_to_scroll(config.drag_to_scroll)
        .resizable(config.resizable)
        .title_bar(config.title_bar)
        .movable(config.movable)
        .frame(config.frame.into())
        .show(&context.context, |ui| {
            if let Some(texture) = texture {
                match context.resource.lock().get_ui_texture(&texture) {
                    Ok(texture) => {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().image(
                            texture.id(),
                            rect,
                            egui::Rect {
                                min: pos2(0.0, 0.0),
                                max: pos2(1.0, 1.0),
                            },
                            config
                                .bg_img_color
                                .unwrap_or(LuaColor {
                                    r: 255,
                                    g: 255,
                                    b: 255,
                                    a: 100,
                                })
                                .into(),
                        );
                    }
                    Err(err) => log::error!("load texture failed: {}", err),
                }
            };
            lua.scope(|scope| {
                let ui_ctx = scope.create_userdata(LuaUiContext { ui, resource })?;
                func.call::<()>(ui_ctx)?;
                Ok(())
            })
            .unwrap_or_else(|e| log::error!("create_window CallBack run failed : {}", e));
        });
    Ok(())
}
