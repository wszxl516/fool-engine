mod dslmod;
mod macros;
mod memmod;
mod utils;
use chrono::{Duration, NaiveDate};
use dslmod::DSLModule;
use lazy_static::lazy_static;
use memmod::MemoryModule;
use mlua::{
    AsChunk, Error as LuaError, FromLuaMulti, Function, IntoLuaMulti, Lua, LuaOptions, Result,
    StdLib, Table, Value, Variadic,
};
use parking_lot::Mutex;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use utils::{dump_lua_stack_trace, lua_values_to_json_string};
#[derive(Debug, Clone)]
pub struct LuaBindings {
    pub lua: Lua,
    #[cfg(feature = "debug")]
    script_path: PathBuf,
    mem_mod: MemoryModule,
    dsl_mod: DSLModule,
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
    pub fn new(_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
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
            script_path: _path.into(),
            mem_mod: MemoryModule::new(),
            dsl_mod: DSLModule::new(),
        })
    }
    fn init_stdlib(&self) -> Result<()> {
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
    fn enable_debug(&self) -> anyhow::Result<()> {
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
            "create_function log_print"
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
            "create_function debug_info"
        )?;
        map2anyhow_error!(
            self.lua.globals().set("debug_info", &debug_info),
            "globals set debug_info"
        )?;

        map2anyhow_error!(
            self.lua.globals().set("__logger", log_print),
            "globals set __logger"
        )?;
        let print = map2anyhow_error!(
            self.lua.create_function(move |_, value: Variadic<Value>| {
                log::debug!("{}", lua_values_to_json_string(value)?);
                Ok(())
            }),
            "create_function debug"
        )?;
        map2anyhow_error!(self.lua.globals().set("print", print), "globals set print")?;
        let stack_trace = map2anyhow_error!(
            self.lua.create_function(move |lua, ()| {
                dump_lua_stack_trace(lua);
                Ok(())
            }),
            "create_function stack_trace"
        )?;
        map2anyhow_error!(
            self.lua.globals().set("stack_trace", stack_trace),
            "globals set stack_trace"
        )?;
        Ok(())
    }
    pub fn setup<K, V, M>(&mut self, modules: M) -> anyhow::Result<()>
    where
        K: Into<String>,
        V: AsRef<[u8]> + Clone,
        M: IntoIterator<Item = (K, V)>,
    {
        map2anyhow_error!(
            self.mem_mod.init(&self.lua, modules),
            "setup_mem_lua failed: {}"
        )?;
        map2anyhow_error!(self.dsl_mod.init(&self.lua), "setup_dsl_lua failed: {}")?;
        #[cfg(feature = "debug")]
        {
            let p: Table = self.lua.globals().get("package").unwrap();
            map2anyhow_error!(
                p.set("path", self.script_path.join("?.lua;")),
                "setup package.path failed"
            )?;
        }
        map2anyhow_error!(self.init_stdlib(), "init_stdlib failed")?;
        self.enable_debug()?;
        Ok(())
    }
    pub fn run<'a>(&self, code: impl AsChunk<'a>, name: impl Into<String>) -> anyhow::Result<()> {
        map2anyhow_error!(
            self.lua.load(code).set_name(name).exec(),
            "run lua code failed"
        )?;
        Ok(())
    }
    pub fn run_dsl_update(&self) {
        self.dsl_mod.run_all_update();
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
    pub fn run_main_fn<R: FromLuaMulti>(
        &self,
        name: &str,
        args: impl IntoLuaMulti,
    ) -> anyhow::Result<R> {
        let func: Function =
            map2anyhow_error!(self.lua.globals().get(name), "get main func failed:")?;
        map2anyhow_error!(func.call::<R>(args), "run main func")
    }
}
