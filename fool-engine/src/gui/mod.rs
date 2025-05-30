use crate::lua::LuaBindings;
use egui::Context;
use mlua::{Function, Lua, UserData, Value};
mod binding;
mod types;
use crate::map2anyhow_error;
use binding::LuaUiContext;
use egui::{pos2, vec2};
use types::LuaUIConfig;
pub struct Gui {
    lua: LuaBindings,
}
use crate::resource::ResourceManager;
use parking_lot::Mutex;
use std::sync::Arc;
pub struct EguiContext {
    pub context: Context,
    pub width: f32,
    pub heigth: f32,
    pub resource: Arc<Mutex<ResourceManager>>,
}
impl UserData for EguiContext {}

impl Gui {
    pub fn new(lua: &LuaBindings) -> Self {
        Self { lua: lua.clone() }
    }
    pub fn init(&self) -> anyhow::Result<()> {
        let create_window_fn =
            move |lua: &Lua, (config, context, func): (LuaUIConfig, Value, Function)| {
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
                        // if let Some(texture) = texture {
                        //     match texture {
                        //         mlua::Value::UserData(ud) => match ud.borrow::<LuaTexture>() {
                        //             Ok(t) => {
                        //                 let rect = ui.available_rect_before_wrap();
                        //                 ui.painter().image(
                        //                     t.ui.id(),
                        //                     rect,
                        //                     Rect {
                        //                         min: pos2(0.0, 0.0),
                        //                         max: pos2(1.0, 1.0),
                        //                     },
                        //                     img_color,
                        //                 );
                        //             }
                        //             Err(err) => log::error!("borrow LuaTextureHandle failed: {}", err),
                        //         },
                        //         _ => {
                        //             log::error!("Wrong LuaTextureHandle type!")
                        //         }
                        //     };
                        // };
                        lua.scope(|scope| {
                            let ui_ctx = scope.create_userdata(LuaUiContext { ui })?;
                            func.call::<()>(ui_ctx)?;
                            Ok(())
                        })
                        .unwrap_or_else(|e| {
                            log::error!("create_window CallBack run failed : {}", e)
                        });
                    });
                Ok(())
            };

        let lua_create_window_fn = map2anyhow_error!(
            self.lua.lua.create_function(create_window_fn),
            "gui_create_window failed"
        )?;
        map2anyhow_error!(
            self.lua
                .lua
                .globals()
                .set("gui_create_window", lua_create_window_fn),
            "gui_create_window failed"
        )
    }
}
