use crate::{map2anyhow_error, map2lua_error};
use mlua::{FromLua, Function, Lua, LuaSerdeExt, Result as LuaResult, Table};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ModKind {
    Init,
}
impl FromLua for ModKind {
    fn from_lua(value: mlua::Value, lua: &Lua) -> LuaResult<Self> {
        lua.from_value(value)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct DSLID {
    pub name: String,
    pub kind: ModKind,
}
impl Display for DSLID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{name: {}, kind: {:?}}}", self.name, self.kind)
    }
}
#[derive(Debug, Clone)]
pub struct DSLContent {
    pub state: Table,
    pub init: Function,
    pub update: Function,
}
impl DSLContent {
    pub fn run_init(&self) -> anyhow::Result<()> {
        map2anyhow_error!(
            self.init.call::<()>(self.state.clone()),
            "run dsl func init failed:"
        )?;
        Ok(())
    }
    pub fn run_update(&self) -> anyhow::Result<()> {
        map2anyhow_error!(
            self.update.call::<()>(self.state.clone()),
            "run dsl func update failed:"
        )?;
        Ok(())
    }
}
#[derive(Debug, Clone, Default)]
pub struct DSLModule {
    pub modules: Arc<Mutex<HashMap<DSLID, DSLContent>>>,
}

impl DSLModule {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init(&mut self, lua: &Lua) -> anyhow::Result<()> {
        let modules = self.modules.clone();
        let register_module_fn = map2anyhow_error!(
            lua.create_function(move |_lua, table: Table| {
                let (mod_id, mod_content) = Self::dsl_from_table(&table)?;
                match mod_content.run_init() {
                    Ok(_) => {
                        log::debug!("finished init module {}", mod_id);
                        modules.lock().insert(mod_id, mod_content);
                    }
                    Err(err) => {
                        log::error!("init module {} failed: {}", mod_id, err);
                    }
                }
                Ok(())
            }),
            "register module failed:"
        )?;
        map2anyhow_error!(
            lua.globals().set("register_module", register_module_fn),
            "set register_module failed:"
        )?;
        Ok(())
    }
    fn dsl_from_table(table: &Table) -> LuaResult<(DSLID, DSLContent)> {
        let mod_name: String = map2lua_error!(table.get("name"), "Incorrect type of name")?;
        let mod_kind: ModKind = map2lua_error!(table.get("kind"), "Incorrect type of kind")?;
        let mod_state: Table = map2lua_error!(table.get("state"), "Incorrect type of state")?;
        let init_func: Function =
            map2lua_error!(table.get("init"), "Incorrect type of init function")?;
        let update_func: Function =
            map2lua_error!(table.get("update"), "Incorrect type of update function")?;
        Ok((
            DSLID {
                name: mod_name,
                kind: mod_kind,
            },
            DSLContent {
                state: mod_state,
                init: init_func,
                update: update_func,
            },
        ))
    }
    pub fn run_all_update(&self) {
        let modules = self.modules.lock();
        for (id, content) in modules.iter() {
            match content.run_update() {
                Ok(_) => {
                    log::debug!("run module {} update fn finished!", id)
                }
                Err(err) => log::error!("run module {} update fn, failed: {}", id, err),
            }
        }
    }
}
