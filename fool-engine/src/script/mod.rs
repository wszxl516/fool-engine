pub mod graphics;
pub mod gui;
pub mod types;
use crate::engine::ResourceManager;
use crate::event::InputEvent;
use crate::resource::lua::LuaResourceManager;
use crate::{map2anyhow_error, physics::LuaPhysics};
use fool_window::WinEvent;
use graphics::window::LuaWindow;
pub use gui::EguiContext;
use lazy_static::lazy_static;
use mlua::{Function, Lua, Result};
use parking_lot::Mutex;
use std::time::Instant;
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

pub fn run_init_fn(lua: &Lua, ctx: &EguiContext, lua_win: &mut LuaWindow) -> anyhow::Result<()> {
    match lua.globals().get::<Function>("init") {
        Ok(init_fn) => {
            map2anyhow_error!(
                lua.scope(|scope| {
                    let window = scope.create_userdata(lua_win)?;
                    let ui_context = lua.create_userdata(ctx.clone())?;
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

pub fn run_frame_fn(
    lua: &Lua,
    ctx: &EguiContext,
    lua_win: &mut LuaWindow,
    events: &Vec<WinEvent>,
) -> anyhow::Result<()> {
    let elapsed = time_peer_frame();
    map2anyhow_error!(
        lua.scope(|scope| {
            let window = scope.create_userdata(lua_win)?;
            let ui_context = lua.create_userdata(ctx.clone())?;
            let input_event = InputEvent { events };
            let input_event = scope.create_userdata(input_event)?;
            let lua_view_fn: Function = lua.globals().get("run_frame")?;
            lua_view_fn.call::<()>((window, ui_context, input_event, elapsed))?;
            Ok(())
        }),
        "run_view_fn failed"
    )
}

pub fn run_event_fn(lua: &Lua, events: &Vec<WinEvent>, lua_win: &mut LuaWindow) -> Result<()> {
    let elapsed = time_peer_frame();
    lua.scope(|scope| {
        let window = scope.create_userdata(lua_win)?;
        let input_event = InputEvent { events };
        let input = scope.create_userdata(input_event)?;
        let lua_event_fn: Function = lua.globals().get("event")?;
        lua_event_fn.call::<()>((input, window, elapsed))?;
        Ok(())
    })
}

pub fn setup_modules(lua: &Lua, res_mgr: ResourceManager) -> anyhow::Result<()> {
    map2anyhow_error!(
        lua.globals()
            .set("ResourceManager", LuaResourceManager::new(res_mgr.clone())),
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
