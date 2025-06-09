pub use super::task::LuaTask;
use crate::FoolScript;
use crate::modules::{DSLID, Modules};
use bson::Bson;
use rayon::{ThreadPool, ThreadPoolBuilder, prelude::*};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
pub type StateMap = Arc<HashMap<DSLID, Bson>>;
pub type ExecutionResults = HashMap<DSLID, anyhow::Result<Bson>>;
#[derive(Debug, Clone)]
pub struct AsyncScheduler {
    pool: Arc<ThreadPool>,
    script: FoolScript,
    running: Arc<AtomicBool>,
}

impl AsyncScheduler {
    pub fn new(script: &FoolScript, thread_num: usize) -> Self {
        let pool = ThreadPoolBuilder::new()
            .num_threads(thread_num)
            .thread_name(|n| format!("LuaThread: {}", n))
            .build()
            .expect("Failed to build custom thread pool");
        Self {
            script: script.clone(),
            pool: Arc::new(pool),
            running: Arc::new(AtomicBool::new(false)),
        }
    }
    pub(crate) fn tasks(
        tasks: &Vec<LuaTask>,
        pool: Arc<ThreadPool>,
        modules: &Modules,
        state_map: &StateMap,
    ) -> ExecutionResults {
        pool.install(|| {
            tasks
                .par_iter()
                .map(|task| {
                    let script_copy = match FoolScript::setup_from_modules(&modules) {
                        Ok(s) => s,
                        Err(e) => {
                            return (task.id.clone(), Err(e));
                        }
                    };
                    task.run(&script_copy, state_map)
                })
                .collect::<ExecutionResults>()
        })
    }
    pub(crate) fn state_map(&self) -> StateMap {
        Arc::new(
            self.script
                .modules
                .dsl_mod
                .modules
                .read()
                .iter()
                .filter_map(|(id, content)| content.get_state().ok().map(|s| (id.clone(), s)))
                .collect(),
        )
    }
    pub fn run(&self) -> anyhow::Result<()> {
        let now = std::time::Instant::now();
        let modules = self.script.modules.clone();
        let dsl_modules = self.script.modules.dsl_mod.clone();
        let pool = self.pool.clone();
        let state_map = self.state_map();
        let tasks = LuaTask::collect_from(&dsl_modules);
        self.running.store(true, Ordering::SeqCst);
        let result_map = Self::tasks(&tasks, pool, &modules, &state_map);
        let mut modules_lock = dsl_modules.modules.write();
        for (id, new_state) in result_map {
            if let Some(m) = modules_lock.get_mut(&id) {
                match new_state {
                    Ok(state) => match m.set_state(&self.script, state) {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(err);
                        }
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
        log::debug!("all thread finished elapsed: {:?}", now.elapsed());
        Ok(())
    }
}
