use crate::FoolScript;
use crate::modules::{DSLContent, DSLID, DSLModule, ser};
use bson::Bson;
use crossbeam_channel::{Receiver, Sender, bounded};
use rayon::{ThreadPool, ThreadPoolBuilder, prelude::*};
use std::collections::HashMap;
use std::sync::Arc;

pub struct LuaTask {
    id: DSLID,
    module: String,
    state: Bson,
}

impl LuaTask {
    pub fn from(id: &DSLID, content: &DSLContent) -> anyhow::Result<Self> {
        Ok(Self {
            id: id.clone(),
            module: content.name(),
            state: content.get_state()?,
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
    rx: Receiver<()>,
    tx: Sender<()>,
    pool: Arc<ThreadPool>,
    script: FoolScript,
}

impl AsyncScheduler {
    pub fn new(script: &FoolScript, thread_num: usize) -> Self {
        let (tx, rx) = bounded::<()>(1);
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

    pub(crate) fn tasks(tasks: &Vec<LuaTask>, script: &FoolScript) -> HashMap<DSLID, Option<Bson>> {
        let results = tasks
            .par_iter()
            .map(|task| {
                let script_other = match FoolScript::setup_modules_from(script) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("Failed to setup script for {:?}: {}", task.id, e);
                        return (task.id.clone(), None);
                    }
                };
                let name = format!("{}, {}", task.id, task.module);
                log::debug!("run mod {}", name);
                let result = match ser::bson_to_lua_value(&script_other.lua, task.state.clone()) {
                    Ok(args) => {
                        let _ = script_other.run_module_fun::<()>(
                            &task.module,
                            &"update".to_owned(),
                            args.clone(),
                        );
                        let res = ser::lua_value_to_bson(args).unwrap_or(Bson::Null);
                        log::error!(" run lua task {}: {:?}", task.id, res);
                        Some(res)
                    }
                    Err(err) => {
                        log::error!("failed to run lua task {}: {}", task.id, err);
                        None
                    }
                };
                log::debug!("finished mod {}", name);
                (task.id.clone(), result)
            })
            .collect::<HashMap<DSLID, Option<Bson>>>();
        results
    }
    pub fn run(&self) {
        let script = self.script.clone();
        let modules = self.script.dsl_mod.clone();
        let tx = self.tx.clone();
        let pool = self.pool.clone();
        std::thread::spawn(move || {
            let tasks = LuaTask::collect_from(&modules);
            let res = pool.install(|| Self::tasks(&tasks, &script));
            let mut lock = modules.modules.write();
            for (id, state) in res {
                if let (Some(m), Some(s)) = (lock.get_mut(&id), state) {
                    if let Err(err) = m.set_state(&script.lua, s) {
                        log::error!("update {} state failed: {}", id, err)
                    } else {
                        log::debug!("finished update {} state", id)
                    }
                }
            }
            let _ = tx.send(());
        });
    }
    pub fn wait_all(&self) {
        let _ = self.rx.recv();
    }
}
