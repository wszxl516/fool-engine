use super::thread::StateMap;
use crate::FoolScript;
use crate::modules::{DSLContent, DSLID, DSLModule, ser};
use anyhow::Result;
use bson::Bson;
use mlua::{Lua, MetaMethod, Table, Value};
#[derive(Debug, Clone)]
pub struct LuaTask {
    pub id: DSLID,
    pub module: String,
    pub state: Bson,
    pub deps: Vec<DSLID>,
}

impl LuaTask {
    pub fn from(id: &DSLID, content: &DSLContent) -> Result<Self> {
        let deps = content.deps.clone();
        Ok(Self {
            id: id.clone(),
            module: content.name(),
            state: content.get_state()?,
            deps,
        })
    }

    pub fn collect_from(modules: &DSLModule) -> Vec<Self> {
        modules
            .modules
            .read()
            .iter()
            .filter_map(|(id, content)| LuaTask::from(id, content).ok())
            .collect()
    }
    fn make_readonly_table(lua: &Lua, orig: Table) -> Result<Table> {
        let proxy = lua.create_table()?;
        for pair in orig.clone().pairs::<Value, Value>() {
            let (k, v) = pair?;
            if let Value::Table(t) = v {
                orig.set(k, Self::make_readonly_table(lua, t)?)?
            }
        }
        let orig_ref = lua.create_registry_value(orig.clone())?;
        let index_func = lua.create_function(move |lua, (_table, key): (Value, Value)| {
            let orig = lua.registry_value::<Table>(&orig_ref)?;
            orig.get::<Value>(key)
        })?;
        let newindex_func =
            lua.create_function(|_, (_table, key, _val): (Value, Value, Value)| {
                Err::<(), mlua::Error>(mlua::Error::RuntimeError(format!(
                    "Cannot assign to readonly field: {:?}",
                    key
                )))
            })?;

        let mt = lua.create_table()?;
        mt.set(MetaMethod::Index.name(), index_func)?;
        mt.set(MetaMethod::NewIndex.name(), newindex_func)?;
        proxy.set_metatable(Some(mt));

        Ok(proxy)
    }
    fn prepare_context(&self, lua: &mlua::Lua, state_map: &StateMap) -> Result<mlua::Value> {
        let ctx = lua.create_table()?;

        if let Some(s) = state_map.get(&self.id) {
            let val = ser::bson_to_lua_value(lua, s.clone())?;
            ctx.set("self", val)?;
        }
        for dep in &self.deps {
            if let Some(dep_val) = state_map.get(dep) {
                let val = ser::bson_to_lua_value(lua, dep_val.clone())?;
                if let Value::Table(tbl) = val {
                    let readonly = Self::make_readonly_table(lua, tbl)?;
                    ctx.set(dep.name.clone(), readonly)?;
                } else {
                    ctx.set(dep.name.clone(), val)?; // fallback
                }
            }
        }
        Ok(mlua::Value::Table(ctx))
    }
    pub fn run(&self, script: &FoolScript, state_map: &StateMap) -> (DSLID, anyhow::Result<Bson>) {
        let ctx = match self.prepare_context(&script, &state_map) {
            Ok(v) => v,
            Err(e) => {
                return (self.id.clone(), Err(e));
            }
        };
        match script.run_module_fun::<()>(&self.module, &"update".to_owned(), ctx.clone()) {
            Ok(_) => (),
            Err(e) => {
                return (self.id.clone(), Err(e));
            }
        };
        let new_state = match ctx {
            mlua::Value::Table(t) => t.get::<mlua::Value>("self").map_err(anyhow::Error::msg),
            _ => Err(anyhow::anyhow!("Expected Table context")),
        };
        let final_bson =
            new_state.and_then(|v| ser::lua_value_to_bson(v).map_err(anyhow::Error::msg));

        (self.id.clone(), final_bson)
    }
}
