#[cfg(not(feature = "debug"))]
pub mod memmod;
pub mod types;
use crate::graphics::window::LuaWindow;
use crate::resource::lua::LuaResourceManager;
use crate::resource::ResourceManager;
use crate::{event::EventState, gui::EguiContext, map2anyhow_error, physics::LuaPhysics};
use chrono::{Duration, NaiveDate};
use egui::Context;
use lazy_static::lazy_static;
#[cfg(not(feature = "debug"))]
use memmod::MemoryModule;
use mlua::{Error as LuaError, Function, Lua, LuaOptions, Result, StdLib, Table, Value, Variadic};
use parking_lot::Mutex;
#[cfg(feature = "debug")]
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use winit::window::Window;
#[derive(Debug, Clone)]
pub struct LuaBindings {
    pub lua: Lua,
    #[cfg(feature = "debug")]
    pub script_path: PathBuf,
    #[cfg(not(feature = "debug"))]
    mem_mod: MemoryModule,
    pub exit: Arc<Mutex<bool>>,
}
lazy_static! {
    static ref start_time: Instant = Instant::now();
    static ref last_time: Mutex<Instant> = Mutex::new(Instant::now());
}

pub fn time_peer_frame() -> f64 {
    let mut lt = last_time.lock();
    let now = Instant::now();
    let dt = now.duration_since(*lt).as_secs_f64();
    *lt = now;
    dt
}
impl LuaBindings {
    pub fn new() -> anyhow::Result<Self> {
        let lua = map2anyhow_error!(
            Lua::new_with(
                StdLib::COROUTINE
                    | StdLib::STRING
                    | StdLib::TABLE
                    | StdLib::MATH
                    | StdLib::PACKAGE
                    | StdLib::UTF8,
                LuaOptions::default(),
            ),
            "init lua failed"
        )?;
        Ok(Self {
            lua,
            #[cfg(feature = "debug")]
            script_path: crate::resource::resource_path()?,
            #[cfg(not(feature = "debug"))]
            mem_mod: MemoryModule::new(),
            exit: Arc::new(Mutex::new(false)),
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
            .create_function(|_, (): ()| Ok(start_time.elapsed().as_secs_f64()))?;
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
                .create_function(move |_lua, (level, value): (String, Value)| {
                    match log::Level::from_str(&level) {
                        Ok(l) => {
                            log::log!(l, "{}", value.to_string()?)
                        }
                        Err(_) => {
                            log::debug!("{}", value.to_string()?)
                        }
                    }
                    Ok(())
                }),
            "create_function debug"
        )?;
        let debug_info = map2anyhow_error!(
            self.lua.create_function(move |lua, value: usize| {
                let res = match lua.inspect_stack(value) {
                    None => ("".to_string(), 0, "".to_owned()),
                    Some(i) => {
                        let name = i.names().name.unwrap_or("<anonymous>".into()).to_string();
                        (
                            name,
                            i.curr_line(),
                            i.source().source.unwrap_or("<unknown>".into()).to_string(),
                        )
                    }
                };
                let t = lua.create_table()?;
                t.set("func", res.0)?;
                t.set("line", res.1)?;
                t.set("file", res.2)?;
                Ok(t)
            }),
            "create_function debug"
        )?;
        map2anyhow_error!(
            self.lua.globals().set("debug_info", &debug_info),
            "globals set debug"
        )?;

        map2anyhow_error!(
            self.lua.globals().set("__logger", log_print),
            "globals set debug"
        )?;
        let print = map2anyhow_error!(
            self.lua.create_function(move |_, value: Variadic<Value>| {
                log::debug!("{}", lua_values_to_json_string(value)?);
                Ok(())
            }),
            "create_function debug"
        )?;
        map2anyhow_error!(self.lua.globals().set("print", print), "globals set print")?;
        Ok(())
    }
    pub fn setup(&mut self, res_mgr: Arc<Mutex<ResourceManager>>) -> anyhow::Result<()> {
        #[cfg(not(feature = "debug"))]
        map2anyhow_error!(
            self.mem_mod.init(&self.lua, res_mgr.clone()),
            "setup_mem_lua failed: {}"
        )?;
        #[cfg(feature = "debug")]
        {
            let p: Table = self.lua.globals().get("package").unwrap();
            map2anyhow_error!(
                p.set("path", self.script_path.join("?.lua;")),
                "setup package.path failed"
            )?;
        }
        map2anyhow_error!(self.disable_module(), "disable_module failed")?;

        map2anyhow_error!(self.physics_module(), "setup lua module physics failed: {}")?;
        self.enable_debug()?;
        map2anyhow_error!(
            self.lua
                .globals()
                .set("ResourceManager", LuaResourceManager::new(res_mgr.clone())),
            "setup lua module EngineTools failed: {}"
        )?;
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
    pub fn run_view_fn(
        &self,
        context: Context,
        resource: Arc<Mutex<ResourceManager>>,
        window: Arc<winit::window::Window>,
    ) -> anyhow::Result<()> {
        let size = window.inner_size();
        map2anyhow_error!(
            self.lua.scope(|scope| {
                let window = scope.create_userdata(LuaWindow {
                    window: window,
                    resource: resource.clone(),
                })?;
                let ui_context = self.lua.create_userdata(EguiContext {
                    context,
                    width: size.width as f32,
                    heigth: size.height as f32,
                    resource,
                })?;
                let lua_view_fn: Function = self.lua.globals().get("view")?;
                lua_view_fn.call::<()>((window, ui_context))?;
                Ok(())
            }),
            "run_view_fn failed"
        )
    }
    pub fn run_update_fn(&self) -> anyhow::Result<()> {
        let elapsed = time_peer_frame();
        map2anyhow_error!(
            self.lua.scope(|_scope| {
                let lua_update_fn: Function = self.lua.globals().get("update")?;
                lua_update_fn.call::<()>(elapsed)?;
                Ok(())
            }),
            "run_update_fn failed"
        )
    }
    pub fn run_init_fn(
        &self,
        ctx: &Context,
        win: Arc<Window>,
        resource: Arc<Mutex<ResourceManager>>,
    ) -> anyhow::Result<()> {
        let size = win.inner_size();
        match self.lua.globals().get::<Function>("init") {
            Ok(init_fn) => {
                map2anyhow_error!(
                    self.lua.scope(|_| {
                        let window = self.lua.create_userdata(LuaWindow {
                            window: win,
                            resource: resource.clone(),
                        })?;
                        let ui_context = self.lua.create_userdata(EguiContext {
                            context: ctx.clone(),
                            width: size.width as f32,
                            heigth: size.height as f32,
                            resource,
                        })?;
                        if let Err(err) = init_fn.call::<()>((window, ui_context)) {
                            log::error!("call lua init fn failed: {}", err);
                        }
                        Ok(())
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
    pub fn run_event_fn(&self, input: &mut EventState) -> Result<()> {
        let elapsed = time_peer_frame();
        self.lua.scope(|scope| {
            let input = scope.create_userdata(input)?;
            let lua_event_fn: Function = self.lua.globals().get("event")?;
            lua_event_fn.call::<()>((input, elapsed))?;
            Ok(())
        })
    }
    pub fn load_main(&self) -> anyhow::Result<()> {
        #[cfg(feature = "debug")]
        {
            use std::io::Read;
            let script = self.script_path.join("main.lua");
            let mut fd = map2anyhow_error!(std::fs::File::open(&script), "load main.lua failed")?;
            let mut script = String::new();
            fd.read_to_string(&mut script)?;
            map2anyhow_error!(self.lua.load(&script).exec(), "run main.lua failed")
        }
        #[cfg(not(feature = "debug"))]
        map2anyhow_error!(
            self.lua.load("require(\"main\")").exec(),
            "run require(\"main\") failed"
        )
    }
}

use serde_json::Value as JsonValue;

fn lua_values_to_json_string(values: Variadic<Value>) -> Result<String> {
    fn convert(value: Value) -> Result<JsonValue> {
        Ok(match value {
            Value::Nil => JsonValue::Null,
            Value::Boolean(b) => JsonValue::Bool(b),
            Value::Integer(i) => JsonValue::Number(i.into()),
            Value::Number(n) => serde_json::Number::from_f64(n)
                .map(JsonValue::Number)
                .unwrap_or(JsonValue::Null),
            Value::String(s) => JsonValue::String(s.to_str()?.to_string()),
            Value::Table(t) => {
                let is_array = t
                    .clone()
                    .pairs::<Value, Value>()
                    .all(|r| matches!(r, Ok((Value::Integer(i), _)) if i >= 1));

                if is_array {
                    let mut vec = Vec::new();
                    for v in t.sequence_values::<Value>() {
                        vec.push(convert(v?)?);
                    }
                    JsonValue::Array(vec)
                } else {
                    let mut map = serde_json::Map::new();
                    for entry in t.pairs::<Value, Value>() {
                        let (k, v) = entry?;
                        let key = match k {
                            Value::String(s) => s.to_str()?.to_string(),
                            Value::Integer(i) => i.to_string(),
                            Value::Number(n) => n.to_string(),
                            _ => continue,
                        };
                        map.insert(key, convert(v)?);
                    }
                    JsonValue::Object(map)
                }
            }
            _ => JsonValue::String(format!("{:?}", value)),
        })
    }

    let mut json_array = String::new();
    for v in values {
        json_array.push_str(&serde_json::to_string_pretty(&convert(v)?).unwrap_or_default());
        json_array.push_str(", ");
    }
    Ok(json_array)
}

pub fn dump_lua_stack_trace(lua: &Lua) {
    log::error!("--- Lua Stack Trace ---");
    let mut level = 0;
    while let Some(debug) = lua.inspect_stack(level) {
        let name = debug.names().name.unwrap_or("<anonymous>".into());
        let source = debug.source().source.unwrap_or("<unknown>".into());
        let line = debug.curr_line();
        let what = debug.event();

        log::error!("#{:<2} [{:?}] {}:{}:{}", level, what, source, line, name);
        level += 1;
    }
    log::error!("--- Lua Stack Trace ---");
}
