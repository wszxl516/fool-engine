use super::graphics::draw::LuaScene;
use super::graphics::sprite::{LuaSrpite, Sprite};
use super::gui::EguiContext;
use super::gui::{create_window, LuaUIConfig};
use super::types::{LuaPoint, LuaSize};
use crate::engine::event::EngineEvent;
use crate::engine::ResourceManager;
use crate::map2lua_error;
use chrono::{Local, Utc};
use egui::Context;
use fool_graphics::canvas::SceneGraph;
use fool_window::{AppEvent, CustomEvent, EventProxy, WindowCursor};
use mlua::{Function, UserData, UserDataMethods};
use parking_lot::RwLock;
use std::path::PathBuf;
use std::{str::FromStr, sync::Arc};
use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize, Position, Size},
    window::{CursorGrabMode, CursorIcon, Fullscreen, Window},
};
#[derive(Clone)]
pub struct LuaEngine {
    pub window: LuaWindow,
    pub ui_ctx: EguiContext,
    pub scene_graph: Arc<RwLock<SceneGraph>>,
}

impl LuaEngine {
    pub fn new(
        window: Arc<Window>,
        context: Context,
        proxy: EventProxy,
        resource: ResourceManager,
        scene_graph: Arc<RwLock<SceneGraph>>,
    ) -> Self {
        let size = window.inner_size();
        let ui_ctx = EguiContext {
            context: context,
            width: size.width as _,
            heigth: size.height as _,
            resource: resource.clone(),
        };
        let window = LuaWindow {
            window: window,
            resource: resource,
            proxy: proxy,
            on_exit: Arc::new(RwLock::new(None)),
        };
        Self {
            window,
            ui_ctx,
            scene_graph,
        }
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        self.ui_ctx.resize(w, h);
    }
}
impl UserData for LuaEngine {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("ui_ctx", |_, this| Ok(this.ui_ctx.clone()));
        fields.add_field_method_get("window", |_, this| Ok(this.window.clone()));
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "draw_window",
            |lua, this, (config, func): (LuaUIConfig, Function)| {
                create_window(lua, config, this.ui_ctx.clone(), func)
            },
        );
        methods.add_method("draw_shape", |_lua, this, scene: LuaScene| {
            let node = scene.0;
            this.scene_graph.write().root.add_child(&node);
            Ok(())
        });
        methods.add_method(
            "create_sprite",
            |_lua, this, (image, frame_size, num): (String, LuaSize<u32>, usize)| {
                let img =
                    map2lua_error!(this.ui_ctx.resource.raw_image.get(image), "create_sprite")?;
                let sprite = Sprite::from_image(img, frame_size.width, frame_size.height, 0..num);
                Ok(LuaSrpite {
                    sprite: sprite,
                    scene_graph: this.scene_graph.clone(),
                })
            },
        );
    }
}

#[derive(Clone)]
pub struct LuaWindow {
    pub window: Arc<Window>,
    pub resource: ResourceManager,
    pub proxy: EventProxy,
    pub on_exit: Arc<RwLock<Option<Function>>>,
}

impl UserData for LuaWindow {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("set_fps", |_lua, this, fps: u32| {
            log::trace!("set_fps to: {}", fps);
            let event: Box<dyn CustomEvent> = Box::new(EngineEvent::FPS(fps));
            map2lua_error!(
                this.proxy.send(AppEvent::CustomEvent(event)),
                "LuaWindow set_fps"
            )?;
            Ok(())
        });
        methods.add_method("capture", |_lua, this, ()| {
            let capture_path = PathBuf::from(format!(
                "{}.png",
                Utc::now()
                    .with_timezone(&Local)
                    .format("%Y-%m-%d-%H-%M-%S%.3f")
            ));
            log::trace!("new capture: {}", capture_path.display());
            let event: Box<dyn CustomEvent> = Box::new(EngineEvent::Capture(capture_path));
            map2lua_error!(
                this.proxy.send(AppEvent::CustomEvent(event)),
                "LuaWindow capture"
            )?;
            Ok(())
        });
        methods.add_method("exit", |_lua, this, ()| {
            map2lua_error!(this.proxy.exit(), "LuaWindow exit")?;
            Ok(())
        });
        methods.add_method_mut("on_exit", |_lua, this, func: Function| {
            this.on_exit.write().replace(func);
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
                    Size::Logical(LogicalSize::new(size.width, size.height)),
                );
                Ok(())
            },
        );

        methods.add_method("set_cursor", |_lua, this, cursor_name: String| {
            let cursor = if let Ok(cursor) = CursorIcon::from_str(&cursor_name) {
                WindowCursor::CursorIcon(cursor)
            } else {
                let img = this.resource.raw_image.get(&cursor_name).map_err(|err| {
                    mlua::Error::RuntimeError(format!(
                        "failed get cursor image {}, {}",
                        &cursor_name, err
                    ))
                })?;
                WindowCursor::Image(img.as_ref().clone())
            };
            map2lua_error!(
                this.proxy.set_cursor(cursor),
                format!("set_cursor to {} failed", cursor_name)
            )?;
            Ok(())
        });

        methods.add_method("set_window_icon", |_lua, this, icon: String| {
            match &this.resource.window_icon.get(&icon) {
                Ok(icon) => this.window.set_window_icon(Some(icon.as_ref().clone())),
                Err(err) => log::error!("failed to get window icon {}, {}", icon, err),
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
        methods.add_method("set_max_size", |_lua, this, size: LuaSize<f64>| {
            this.window
                .set_max_inner_size(Some(PhysicalSize::new(size.width, size.height)));
            Ok(())
        });
        methods.add_method("set_min_size", |_lua, this, size: LuaSize<f64>| {
            this.window
                .set_min_inner_size(Some(PhysicalSize::new(size.width, size.height)));
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
                width: size.width,
                height: size.height,
            })
        });
        methods.add_method("outer_size", |_lua, this, (): ()| {
            let size = this.window.outer_size();
            Ok(LuaSize {
                width: size.width,
                height: size.height,
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
    }
}
