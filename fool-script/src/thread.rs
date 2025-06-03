use crate::FoolScript;
use crate::modules::{DSLContent, DSLID, DSLModule, ser};
use anyhow::Result;
use bson::Bson;
use crossbeam_channel::{Receiver, Sender, bounded};
use mlua::{Lua, MetaMethod, Table, Value};
use rayon::{ThreadPool, ThreadPoolBuilder, prelude::*};
use std::collections::HashMap;
use std::sync::Arc;

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
}

#[derive(Debug, Clone)]
pub struct AsyncScheduler {
    rx: Receiver<anyhow::Result<()>>,
    tx: Sender<anyhow::Result<()>>,
    pool: Arc<ThreadPool>,
    script: FoolScript,
}

impl AsyncScheduler {
    pub fn new(script: &FoolScript, thread_num: usize) -> Self {
        let (tx, rx) = bounded::<anyhow::Result<()>>(1);
        let pool = ThreadPoolBuilder::new()
            .num_threads(thread_num)
            .thread_name(|n| format!("LuaAsyncTask: {}", n))
            .build()
            .expect("Failed to build custom thread pool");
        Self {
            script: script.clone(),
            tx,
            rx,
            pool: Arc::new(pool),
        }
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
    fn prepare_context(
        lua: &mlua::Lua,
        task: &LuaTask,
        state_map: &HashMap<DSLID, Bson>,
    ) -> Result<mlua::Value> {
        let ctx = lua.create_table()?;

        if let Some(s) = state_map.get(&task.id) {
            let val = ser::bson_to_lua_value(lua, s.clone())?;
            ctx.set("self", val)?;
        }
        for dep in &task.deps {
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
    pub(crate) fn tasks(
        tasks: &Vec<LuaTask>,
        pool: Arc<ThreadPool>,
        script: &FoolScript,
        state_map: &HashMap<DSLID, Bson>,
    ) -> HashMap<DSLID, anyhow::Result<Bson>> {
        pool.install(|| {
            tasks
                .par_iter()
                .map(|task| {
                    let script_copy = match FoolScript::setup_modules_from(&script) {
                        Ok(s) => s,
                        Err(e) => {
                            return (task.id.clone(), Err(e));
                        }
                    };

                    let ctx = match Self::prepare_context(&script_copy.lua, task, &state_map) {
                        Ok(v) => v,
                        Err(e) => {
                            return (task.id.clone(), Err(e));
                        }
                    };

                    match script_copy.run_module_fun::<()>(
                        &task.module,
                        &"update".to_owned(),
                        ctx.clone(),
                    ) {
                        Ok(_) => (),
                        Err(e) => {
                            return (task.id.clone(), Err(e));
                        }
                    };

                    let new_state = match ctx {
                        mlua::Value::Table(t) => {
                            t.get::<mlua::Value>("self").map_err(anyhow::Error::msg)
                        }
                        _ => Err(anyhow::anyhow!("Expected Table context")),
                    };

                    let final_bson = new_state
                        .and_then(|v| ser::lua_value_to_bson(v).map_err(anyhow::Error::msg));

                    (task.id.clone(), final_bson)
                })
                .collect::<HashMap<DSLID, anyhow::Result<Bson>>>()
        })
    }
    pub(crate) fn state_map(&self) -> HashMap<DSLID, Bson> {
        self.script
            .dsl_mod
            .modules
            .read()
            .iter()
            .filter_map(|(id, content)| content.get_state().ok().map(|s| (id.clone(), s)))
            .collect()
    }
    pub fn run(&self) {
        let script = self.script.clone();
        let modules = self.script.dsl_mod.clone();
        let tx = self.tx.clone();
        let pool = self.pool.clone();
        let state_map = self.state_map();
        let tasks = LuaTask::collect_from(&modules);
        std::thread::spawn(move || {
            let result_map = Self::tasks(&tasks, pool, &script, &state_map);
            let mut modules_lock = modules.modules.write();
            for (id, new_state) in result_map {
                if let Some(m) = modules_lock.get_mut(&id) {
                    match new_state {
                        Ok(state) => match m.set_state(&script.lua, state) {
                            Ok(_) => {}
                            Err(err) => {
                                log::error!("Failed to update {}: {}", id, err);
                                let _ = tx.send(Err(err));
                                break;
                            }
                        },
                        Err(err) => {
                            log::error!("Failed to run {}: {}", id, err);
                            let _ = tx.send(Err(err));
                            break;
                        }
                    }
                }
            }
            let _ = tx.send(Ok(()));
        });
    }

    pub fn wait_all(&self) -> anyhow::Result<()> {
        self.rx.recv()?
    }
}
