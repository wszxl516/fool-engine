use std::{io::Read, path::PathBuf};
pub mod types;
pub mod zipmod;
use super::graphics::window::LuaWindow;
use crate::resource::lua::LuaResourceManager;
use crate::resource::ResourceManager;
use crate::{
    graphics::draw::LuaCancvas, gui::EguiContext, input::InputState, map2anyhow_error,
    physics::LuaPhysics,
};
use chrono::{Duration, NaiveDate};
use lazy_static::lazy_static;
use mlua::{Error as LuaError, Function, Lua, LuaOptions, Result, StdLib, Table, Value};
use nannou::{App, Draw};
use nannou_egui::egui::Context;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use zipmod::LuaZipModule;
#[derive(Debug, Clone)]
pub struct LuaBindings {
    pub lua: Lua,
    pub script_path: PathBuf,
    zip_mod: LuaZipModule,
}
lazy_static! {
    static ref globals_Instant: Instant = Instant::now();
}
impl LuaBindings {
    pub fn new(script_path: &PathBuf) -> Result<Self> {
        let lua = Lua::new_with(
            StdLib::COROUTINE
                | StdLib::STRING
                | StdLib::TABLE
                | StdLib::MATH
                | StdLib::PACKAGE
                | StdLib::UTF8,
            LuaOptions::default(),
        )?;
        Ok(Self {
            lua,
            script_path: script_path.clone(),
            zip_mod: LuaZipModule::new(),
        })
    }
    fn disable_module(&self) -> Result<()> {
        let os_table = self.lua.create_table()?;

        // os.date
        let date = self.lua.create_function(|_, format: Option<String>| {
            let format = format.unwrap_or("%c".to_string());
            Ok(chrono::Local::now().format(&format).to_string())
        })?;
        // os.clock
        let clock = self
            .lua
            .create_function(|_, (): ()| Ok(globals_Instant.elapsed().as_secs_f64()))?;
        // os.time
        let time = self.lua.create_function(|_lua, arg: Option<Table>| {
            let timestamp = match arg {
                None => {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                    now.as_secs() as i64
                }
                Some(t) => {
                    let year: i32 = t.get("year")?;
                    let month: u32 = t.get("month")?;
                    let day: u32 = t.get("day")?;
                    let hour: i64 = t.get("hour").unwrap_or(12);
                    let min: i64 = t.get("min").unwrap_or(0);
                    let sec: i64 = t.get("sec").unwrap_or(0);
                    let base = NaiveDate::from_ymd_opt(year, month, day)
                        .ok_or_else(|| LuaError::RuntimeError("invalid date".to_owned()))?;
                    let base_dt = base
                        .and_hms_opt(0, 0, 0)
                        .ok_or_else(|| LuaError::RuntimeError("invalid time".to_owned()))?;

                    let offset =
                        Duration::hours(hour) + Duration::minutes(min) + Duration::seconds(sec);
                    let final_dt = base_dt + offset;
                    final_dt.and_utc().timestamp()
                }
            };

            Ok(Value::Integer(timestamp))
        })?;

        //  os.difftime
        let difftime = self
            .lua
            .create_function(|_, (t1, t2): (i64, i64)| Ok(t1 - t2))?;
        os_table.set("clock", clock)?;
        os_table.set("date", date)?;
        os_table.set("time", time)?;
        os_table.set("difftime", difftime)?;
        os_table.set("execute", Value::Nil)?;
        os_table.set("exit", Value::Nil)?;
        os_table.set("getenv", Value::Nil)?;
        os_table.set("remove", Value::Nil)?;
        os_table.set("rename", Value::Nil)?;
        os_table.set("setlocale", Value::Nil)?;
        self.lua.globals().set("os", os_table)?;
        Ok(())
    }
    pub fn enable_debug(&self) -> anyhow::Result<()> {
        let log_print = map2anyhow_error!(
            self.lua
                .create_function(move |_, (level, value): (String, Value)| {
                    match log::Level::from_str(&level) {
                        Ok(l) => {
                            log::log!(l, "{}\n", value.to_string()?)
                        }
                        Err(_) => {}
                    }
                    Ok(())
                }),
            "create_function debug"
        );
        let debug_info = map2anyhow_error!(
            self.lua.create_function(move |lua, value: usize| {
                use std::borrow::Cow;
                let res = match lua.inspect_stack(value) {
                    None => ("".to_string(), 0),
                    Some(i) => {
                        let name = i.names().name.unwrap_or(Cow::default()).to_string();
                        (name, i.curr_line())
                    }
                };
                let t = lua.create_table()?;
                t.set("func", res.0)?;
                t.set("line", res.1)?;
                Ok(t)
            }),
            "create_function debug"
        );
        map2anyhow_error!(
            self.lua.globals().set("debug_info", debug_info),
            "globals set debug"
        );

        map2anyhow_error!(
            self.lua.globals().set("__logger", log_print),
            "globals set debug"
        );
        Ok(())
    }
    pub fn setup_zip_lua<I, P>(&mut self, modules: P) -> anyhow::Result<()>
    where
        I: Into<PathBuf> + Sized,
        P: IntoIterator<Item = I>,
    {
        self.zip_mod
            .init(&self.lua, modules)
            .map_err(|err| anyhow::anyhow!("setup_zip_lua failed: {}", err))
    }
    pub fn setup(&self, res_mgr: Arc<Mutex<ResourceManager>>) -> anyhow::Result<()> {
        self.disable_module()
            .map_err(|e| anyhow::anyhow!("disable_module failed: {}", e))?;
        let p: Table = self.lua.globals().get("package").unwrap();
        p.set("path", self.script_path.join("?.lua;")).unwrap();
        self.physics_module()
            .map_err(|e| anyhow::anyhow!(format!("setup lua module physics failed: {}", e)))?;
        self.enable_debug()?;
        self.lua
            .globals()
            .set("ResourceManager", LuaResourceManager::new(res_mgr.clone()))
            .map_err(|e| anyhow::anyhow!(format!("setup lua module EngineTools failed: {}", e)))?;
        Ok(())
    }

    pub fn physics_module(&self) -> Result<()> {
        let physics_init = self.lua.create_function(
            move |_: &Lua, (x_gravity_acceleration, y_gravity_acceleration): (f32, f32)| {
                let physics = LuaPhysics::new(x_gravity_acceleration, y_gravity_acceleration);
                Ok(physics)
            },
        )?;
        self.lua.globals().set("physics_init", physics_init)?;
        Ok(())
    }
    pub fn run_draw_fn(
        &self,
        draw: Draw,
        context: Context,
        app: &App,
        resource: Arc<Mutex<ResourceManager>>,
    ) -> anyhow::Result<()> {
        let window = app
            .window(app.window_id())
            .ok_or_else(|| anyhow::anyhow!("run_draw_fn: get window failed!"))?;
        let window = LuaWindow { window };
        let (w, h) = window.window.inner_size_points();
        let lua_canvas = LuaCancvas::new(draw.clone());
        self.lua
            .scope(|scope| {
                let lua_canvas = scope.create_userdata(lua_canvas)?;
                let window = scope.create_userdata(window)?;
                let ui_context = self.lua.create_userdata(EguiContext {
                    context,
                    width: w,
                    heigth: h,
                    resource,
                })?;
                let lua_view_fn: Function = self.lua.globals().get("view")?;
                lua_view_fn.call::<()>((lua_canvas, ui_context, window))?;
                Ok(())
            })
            .map_err(|err| anyhow::anyhow!("run_draw_fn failed: {}", err))
    }
    pub fn run_update_fn(&self) -> anyhow::Result<()> {
        let lua_update_fn: Function = self
            .lua
            .globals()
            .get("update")
            .map_err(|err| anyhow::anyhow!("run_draw_fn failed: {}", err))?;
        lua_update_fn
            .call::<()>(())
            .map_err(|err| anyhow::anyhow!("run_draw_fn failed: {}", err))
    }
    pub fn run_init_fn(&self) -> Result<()> {
        self.lua.load("if init then init() end").exec()
    }
    pub fn run_event_fn(&self, input: &InputState) -> Result<()> {
        self.lua.scope(|scope| {
            let input = scope.create_userdata(input)?;
            let lua_event_fn: Function = self.lua.globals().get("event")?;
            lua_event_fn.call::<()>(input)?;
            Ok(())
        })
    }
    pub fn load_main(&self, asset: &PathBuf) -> Result<()> {
        let mut fd = std::fs::File::open(asset.join("main.lua"))?;
        let mut script = String::new();
        fd.read_to_string(&mut script)?;
        self.lua.load(script).exec()
    }
}
