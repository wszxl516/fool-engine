use crate::{graphics::types::LuaColor, lua::LuaBindings, resource::types::LuaTexture};
use mlua::{Function, Lua, UserData, Value};
use nannou::prelude::*;
use nannou_egui::{
    self,
    egui::{pos2, vec2, Context, Rect},
    Egui,
};
mod binding;
mod types;
use binding::LuaUiContext;
use std::cell::RefCell;
use std::rc::Rc;
use types::LuaUIConfig;

pub struct Gui {
    pub egui: Rc<RefCell<Egui>>,
    lua: LuaBindings,
}
use crate::resource::ResourceManager;
use std::sync::{Arc, Mutex};
pub struct EguiContext {
    pub context: Context,
    pub width: f32,
    pub heigth: f32,
    pub resource: Arc<Mutex<ResourceManager>>,
}
impl UserData for EguiContext {}

impl Gui {
    pub fn new(window: &nannou::window::Window, lua: &LuaBindings) -> Self {
        let egui = Rc::new(RefCell::new(Egui::from_window(window)));
        Self {
            egui,
            lua: lua.clone(),
        }
    }
    pub fn end(&self, frame: &Frame) -> anyhow::Result<()> {
        self.egui
            .borrow_mut()
            .draw_to_frame(frame)
            .map_err(|e| anyhow::anyhow!("egui Draw to frame failed: {:?}", e))?;
        Ok(())
    }
    pub fn init(&self) {
        let create_window_fn = move |lua: &Lua,
                                     (window, context, texture, color, func): (
            LuaUIConfig,
            Value,
            Option<Value>,
            Option<LuaColor>,
            Function,
        )| {
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
            let img_color = color
                .unwrap_or(LuaColor {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 128,
                })
                .into();
            let x_c = context.width / 2.0 + window.x - window.w / 2.0;
            let y_c = context.heigth / 2.0 - window.y - window.h / 2.0;
            let pos = pos2(x_c, y_c);
            let size = vec2(window.w, window.h);
            nannou_egui::egui::containers::Window::new(window.title)
                .collapsible(window.collapsible)
                .constrain(window.constrain)
                .default_open(window.default_open)
                .default_pos(pos)
                .default_size(size)
                .enabled(true)
                .drag_to_scroll(window.drag_to_scroll)
                .resizable(window.resizable)
                .title_bar(window.title_bar)
                .movable(window.movable)
                .frame(window.frame.into())
                .show(&context.context, |ui| {
                    if let Some(texture) = texture {
                        match texture {
                            mlua::Value::UserData(ud) => match ud.borrow::<LuaTexture>() {
                                Ok(t) => {
                                    let rect = ui.available_rect_before_wrap();
                                    ui.painter().image(
                                        t.ui.id(),
                                        rect,
                                        Rect {
                                            min: pos2(0.0, 0.0),
                                            max: pos2(1.0, 1.0),
                                        },
                                        img_color,
                                    )
                                }
                                Err(err) => log::error!("borrow LuaTextureHandle failed: {}", err),
                            },
                            _ => {
                                log::error!("Wrong LuaTextureHandle type!")
                            }
                        };
                    };
                    lua.scope(|scope| {
                        let ui_ctx = scope.create_userdata(LuaUiContext { ui })?;
                        func.call::<()>(ui_ctx)?;
                        Ok(())
                    })
                    .unwrap_or_else(|e| log::error!("create_window CallBack run failed : {}", e));
                });
            Ok(())
        };

        let lua_create_window_fn = self
            .lua
            .lua
            .create_function(create_window_fn)
            .map_err(|e| anyhow::anyhow!("gui_create_window failed: {:?}", e))
            .unwrap();
        self.lua
            .lua
            .globals()
            .set("gui_create_window", lua_create_window_fn)
            .unwrap();
    }
    pub fn event(&self, event: &nannou::winit::event::WindowEvent) {
        self.egui.borrow_mut().handle_raw_event(event);
    }
}
