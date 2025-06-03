pub mod graphics;
pub mod gui;
pub mod types;
use crate::engine::ResourceManager;
use crate::event::EngineEventLoop;
use crate::resource::lua::LuaResourceManager;
use crate::{event::EventState, map2anyhow_error, physics::LuaPhysics};
use egui::Context;
use graphics::window::LuaWindow;
pub use gui::EguiContext;
use lazy_static::lazy_static;
use mlua::{Function, Lua, Result};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Instant;
use winit::window::Window;
lazy_static! {
    static ref last_time: Mutex<Instant> = Mutex::new(Instant::now());
}

pub fn time_peer_frame() -> f64 {
    let mut lt = last_time.lock();
    let now = Instant::now();
    let dt = now.duration_since(*lt).as_secs_f64();
    *lt = now;
    dt
}

pub fn run_init_fn(
    lua: &Lua,
    ctx: &Context,
    win: Arc<Window>,
    resource: ResourceManager,
    event_loop_proxy: EngineEventLoop,
) -> anyhow::Result<()> {
    let size = win.inner_size();
    match lua.globals().get::<Function>("init") {
        Ok(init_fn) => {
            map2anyhow_error!(
                lua.scope(|_| {
                    let window = lua.create_userdata(LuaWindow {
                        window: win,
                        resource: resource.clone(),
                        event_loop: event_loop_proxy,
                    })?;
                    let ui_context = lua.create_userdata(EguiContext {
                        context: ctx.clone(),
                        width: size.width as f32,
                        heigth: size.height as f32,
                        resource,
                    })?;
                    init_fn.call::<()>((window, ui_context))
                }),
                "run_init_fn"
            )?;
            Ok(())
        }
        Err(err) => {
            log::error!("get lua init func failed: {}", err);
            Ok(())
        }
    }
}

pub fn run_view_fn(
    lua: &Lua,
    context: Context,
    resource: ResourceManager,
    window: Arc<winit::window::Window>,
    event_loop_proxy: EngineEventLoop,
) -> anyhow::Result<()> {
    let size = window.inner_size();
    map2anyhow_error!(
        lua.scope(|scope| {
            let window = scope.create_userdata(LuaWindow {
                window: window,
                resource: resource.clone(),
                event_loop: event_loop_proxy,
            })?;
            let ui_context = lua.create_userdata(EguiContext {
                context,
                width: size.width as f32,
                heigth: size.height as f32,
                resource,
            })?;
            let lua_view_fn: Function = lua.globals().get("view")?;
            lua_view_fn.call::<()>((window, ui_context))?;
            Ok(())
        }),
        "run_view_fn failed"
    )
}

pub fn run_event_fn(
    lua: &Lua,
    input: &mut EventState,
    win: Arc<Window>,
    resource: ResourceManager,
    event_loop_proxy: EngineEventLoop,
) -> Result<()> {
    let elapsed = time_peer_frame();
    lua.scope(|scope| {
        let window = lua.create_userdata(LuaWindow {
            window: win,
            resource: resource.clone(),
            event_loop: event_loop_proxy,
        })?;
        let input = scope.create_userdata(input)?;
        let lua_event_fn: Function = lua.globals().get("event")?;
        lua_event_fn.call::<()>((input, window, elapsed))?;
        Ok(())
    })
}

pub fn run_update_fn(lua: &Lua) -> anyhow::Result<()> {
    let elapsed = time_peer_frame();
    map2anyhow_error!(
        lua.scope(|_scope| {
            let lua_update_fn: Function = lua.globals().get("update")?;
            lua_update_fn.call::<()>(elapsed)?;
            Ok(())
        }),
        "run_update_fn failed"
    )
}

pub fn setup_modules(
    lua: &Lua,
    res_mgr: ResourceManager,
    event_loop: EngineEventLoop,
) -> anyhow::Result<()> {
    map2anyhow_error!(
        lua.globals().set(
            "ResourceManager",
            LuaResourceManager::new(res_mgr.clone(), event_loop)
        ),
        "setup lua module EngineTools failed: {}"
    )?;
    setup_physics(lua)?;
    Ok(())
}
#[inline]
fn setup_physics(lua: &Lua) -> anyhow::Result<()> {
    let physics_init = lua.create_function(
        move |_: &Lua, (x_gravity_acceleration, y_gravity_acceleration): (f32, f32)| {
            let physics = LuaPhysics::new(x_gravity_acceleration, y_gravity_acceleration);
            Ok(physics)
        },
    )?;
    lua.globals().set("physics_init", physics_init)?;
    Ok(())
}
