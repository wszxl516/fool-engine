use super::ser;
use crate::{map2anyhow_error, map2lua_error};
use bson::Bson;
use mlua::{FromLua, Function, Lua, LuaSerdeExt, Result as LuaResult, Table};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct DSLID {
    pub name: String,
}
impl Display for DSLID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DSLID {{name: {} }}", self.name)
    }
}
#[derive(Debug, Clone)]
pub struct DSLContent {
    /// How many frames to execute once
    pub frames_interval: u64,
    pub init: Function,
    pub update: Function,
    pub module: Table,
    pub deps: Vec<DSLID>,
    pub enabled: bool
}
impl DSLContent {
    pub fn state(&self) -> anyhow::Result<Table> {
        map2anyhow_error!(
            self.module.get("shared_state"),
            "failed get shared_state from module!"
        )
    }
    pub fn name(&self) -> String {
        self.module
            .get("__modname")
            .unwrap_or("<anonymous>".to_owned())
    }
    pub fn get_state(&self) -> anyhow::Result<Bson> {
        let bson = map2anyhow_error!(
            ser::lua_value_to_bson(mlua::Value::Table(self.state()?)),
            "Failed to serialize Lua value"
        )?;
        Ok(bson)
    }
    pub fn set_state(&self, lua: &Lua, data: Bson) -> anyhow::Result<()> {
        let state = map2anyhow_error!(
            ser::bson_to_lua_value(lua, data),
            "Deserializa of lua value failed"
        )?;
        map2anyhow_error!(
            self.module.set("shared_state", state),
            "set shared_state failed!"
        )?;
        Ok(())
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Default)]
pub struct DSLModule {
    pub modules: Arc<RwLock<HashMap<DSLID, DSLContent>>>,
}

impl DSLModule {
    pub fn new() -> Self {
        Self {
            modules: Default::default(),
        }
    }
    pub fn init(&mut self, lua: &Lua) -> anyhow::Result<()> {
        let modules = self.modules.clone();
        let register_module_fn = map2anyhow_error!(
            lua.create_function(move |_lua, table: Table| {
                let (mod_id, mod_content) = Self::dsl_from_table(&table)?;
                modules.write().insert(mod_id, mod_content);
                Ok(())
            }),
            "register module failed:"
        )?;
        map2anyhow_error!(
            lua.globals()
                .set("register_threaded_module", register_module_fn),
            "set register_threaded_module failed:"
        )?;
        Ok(())
    }
    fn dsl_from_table(table: &Table) -> LuaResult<(DSLID, DSLContent)> {
        let mod_name: String = map2lua_error!(table.get("name"), "Incorrect type of name")?;
        let frames_interval: u64 = map2lua_error!(
            table.get("frames_interval"),
            "Incorrect type of frames_interval"
        )?;
        let _: Table = map2lua_error!(table.get("shared_state"), "Incorrect type of shared_state")?;
        let enabled: bool = map2lua_error!(table.get("enabled"), "Incorrect type of enabled")?;
        println!("enabled: {}", enabled);
        let init_func: Function =
            map2lua_error!(table.get("init"), "Incorrect type of init function")?;
        let update_func: Function =
            map2lua_error!(table.get("update"), "Incorrect type of update function")?;
        let deps = match table.get::<mlua::Table>("deps") {
            Ok(table) => table
                .sequence_values::<String>()
                .filter_map(|r| r.ok())
                .map(|name| DSLID { name })
                .collect::<Vec<_>>(),
            Err(_) => vec![],
        };
        Ok((
            DSLID { name: mod_name },
            DSLContent {
                frames_interval,
                init: init_func,
                update: update_func,
                module: table.clone(),
                deps,
                enabled
            },
        ))
    }
}
