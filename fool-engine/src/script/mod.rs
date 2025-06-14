pub mod audio;
pub mod engine;
pub mod graphics;
pub mod gui;
pub mod types;
use crate::event::InputEvent;
use crate::{map2anyhow_error, physics::LuaPhysics};
pub use engine::{LuaEngine, LuaWindow};
use fool_script::FoolScript;
use fool_window::WinEvent;
pub use gui::EguiContext;
use lazy_static::lazy_static;
use mlua::{Function, Lua, Value};
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

pub fn run_init_fn(lua: &Lua, lua_win: &LuaEngine) -> anyhow::Result<()> {
    match lua.globals().get::<Function>("init") {
        Ok(init_fn) => {
            map2anyhow_error!(
                lua.scope(|_| {
                    let window = lua.create_userdata(lua_win.clone())?;
                    init_fn.call::<()>(window)
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

pub fn run_fn(lua: &Lua, lua_win: &LuaEngine, events: &Vec<WinEvent>) -> anyhow::Result<()> {
    let elapsed = time_peer_frame();
    map2anyhow_error!(
        lua.scope(|scope| {
            let window = scope.create_userdata(lua_win.clone())?;
            let input_event = InputEvent { events };
            let input_event = scope.create_userdata(input_event)?;
            let lua_view_fn: Function = lua.globals().get("run")?;
            lua_view_fn.call::<()>((window, input_event, elapsed))?;
            Ok(())
        }),
        "run_frame_fn failed"
    )
}
pub fn exit_fn(lua: &Lua, lua_win: &LuaEngine, events: &Vec<WinEvent>) -> anyhow::Result<()> {
    let elapsed = time_peer_frame();
    map2anyhow_error!(
        lua.scope(|scope| {
            let window = scope.create_userdata(lua_win.clone())?;
            let input_event = InputEvent { events };
            let input_event = scope.create_userdata(input_event)?;
            let lua_view_fn: Function = lua.globals().get("exit")?;
            lua_view_fn.call::<()>((window, input_event, elapsed))?;
            Ok(())
        }),
        "exit_fn failed"
    )
}

pub fn pause_fn(lua: &Lua, lua_win: &LuaEngine, events: &Vec<WinEvent>) -> anyhow::Result<()> {
    let elapsed = time_peer_frame();
    map2anyhow_error!(
        lua.scope(|scope| {
            let window = scope.create_userdata(lua_win.clone())?;
            let input_event = InputEvent { events };
            let input_event = scope.create_userdata(input_event)?;
            let lua_view_fn: Function = lua.globals().get("pause")?;
            lua_view_fn.call::<()>((window, input_event, elapsed))?;
            Ok(())
        }),
        "pause_fn failed"
    )
}
pub fn setup_modules(lua: &FoolScript) -> anyhow::Result<()> {
    lua.register_user_mod("Physics", |lua: &Lua| {
        let lua_phy_new = lua.create_function(|_, (x, y): (f32, f32)| Ok(LuaPhysics::new(x, y)))?;
        let lua_phy = lua.create_table()?;
        lua_phy.set("new", lua_phy_new)?;
        Ok(Value::Table(lua_phy))
    })?;
    Ok(())
}
