use crate::{
    map2anyhow_error,
    utils::{dump_lua_stack_trace, lua_values_to_json_string},
};
use chrono::{Duration, NaiveDate};
use lazy_static::lazy_static;
use mlua::{
    AsChunk, Error as LuaError, FromLuaMulti, Function, IntoLuaMulti, Lua, LuaOptions, Result,
    StdLib, Table, Value, Variadic,
};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref start_time: Instant = Instant::now();
}

pub fn init_stdlib(lua: &Lua) -> Result<()> {
    let os_table = lua.create_table()?;

    // os.date
    let date = lua.create_function(|_, format: Option<String>| {
        let format = format.unwrap_or("%c".to_string());
        Ok(chrono::Local::now().format(&format).to_string())
    })?;
    // os.clock
    let clock = lua.create_function(|_, (): ()| Ok(start_time.elapsed().as_secs_f64()))?;
    // os.time
    let time = lua.create_function(|_lua, arg: Option<Table>| {
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
    let difftime = lua.create_function(|_, (t1, t2): (i64, i64)| Ok(t1 - t2))?;
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
    lua.globals().set("os", os_table)?;
    Ok(())
}

pub fn enable_debug(lua: &Lua) -> anyhow::Result<()> {
    let log_print = map2anyhow_error!(
        lua.create_function(move |_lua, (level, value): (String, Value)| {
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
        lua.create_function(move |lua, value: usize| {
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
        lua.globals().set("debug_info", &debug_info),
        "globals set debug_info"
    )?;

    map2anyhow_error!(
        lua.globals().set("__logger", log_print),
        "globals set __logger"
    )?;
    let print = map2anyhow_error!(
        lua.create_function(move |_, value: Variadic<Value>| {
            log::debug!("{}", lua_values_to_json_string(value)?);
            Ok(())
        }),
        "create_function debug"
    )?;
    map2anyhow_error!(lua.globals().set("print", print), "globals set print")?;
    let stack_trace = map2anyhow_error!(
        lua.create_function(move |lua, ()| {
            dump_lua_stack_trace(lua);
            Ok(())
        }),
        "create_function stack_trace"
    )?;
    map2anyhow_error!(
        lua.globals().set("stack_trace", stack_trace),
        "globals set stack_trace"
    )?;
    Ok(())
}
