pub use super::task::{LuaTask, ThreadResponse};
use crate::FoolScript;
use crate::modules::{DSLID, Modules};
use crate::thread::fullchannel::FullChannel;
use bson::Bson;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::JoinHandle;
pub type StateMap = Arc<HashMap<DSLID, Bson>>;

#[derive(Debug)]
pub enum ThreadControl {
    /// (current fame state from main thread, frame_id)
    Start(StateMap, u64),
    Stop,
}
#[derive(Debug)]
pub struct AsyncScheduler {
    modules: Modules,
    task_map: HashMap<DSLID, (JoinHandle<()>, FullChannel<ThreadControl, ThreadResponse>)>,
}

impl AsyncScheduler {
    pub fn new(modules: Modules) -> Self {
        Self {
            modules: modules,
            task_map: Default::default(),
        }
    }
    pub(crate) fn runner(
        task: LuaTask,
        modules: Modules,
        slaver: FullChannel<ThreadResponse, ThreadControl>,
    ) {
        let mut slaver = slaver;
        match FoolScript::setup_from_modules(&modules) {
            Ok(script) => {
                let _ = slaver.sender().send(task.run_init(&script));
                loop {
                    if let Ok(control) = slaver.receiver().recv() {
                        match control {
                            ThreadControl::Start(state_map, frame_id) => {
                                let res = if frame_id % task.frames_interval == 0 {
                                    task.run_update(&script, &state_map)
                                } else {
                                    ThreadResponse {
                                        id: task.id.clone(),
                                        content: Ok(None),
                                    }
                                };
                                let _ = slaver.sender().send(res);
                            }
                            ThreadControl::Stop => break,
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("setup FoolScript env for {}, failed: {}", task.id, err);
            }
        }
    }
    pub(crate) fn start_thread(&mut self, task: LuaTask, modules: Modules) -> anyhow::Result<()> {
        let (master, slave) = FullChannel::<ThreadControl, ThreadResponse>::new(1);
        let task_cloneed = task.clone();
        let res = std::thread::Builder::new()
            .name(format!("Fool-Script"))
            .spawn(move || {
                Self::runner(task_cloneed, modules, slave);
            });
        match res {
            Ok(h) => {
                self.task_map.insert(task.id, (h, master));
                Ok(())
            }
            Err(err) => {
                log::error!("error start {} on thread : {}", task.id, err);
                Err(anyhow::anyhow!("{}", err))
            }
        }
    }
    pub(crate) fn state_map(&self) -> StateMap {
        Arc::new(
            self.modules
                .dsl_mod
                .modules
                .read()
                .iter()
                .filter_map(|(id, content)| content.get_state().ok().map(|s| (id.clone(), s)))
                .collect(),
        )
    }
    pub fn init(&mut self) -> anyhow::Result<()> {
        let modules = self.modules.clone();
        let dsl_modules = self.modules.dsl_mod.clone();
        let tasks = LuaTask::collect_from(&dsl_modules);
        for t in tasks {
            self.start_thread(t.clone(), modules.clone())?;
            if let Some((_j, c)) = self.task_map.get_mut(&t.id) {
                let res = c.receiver().recv()?;
                if res.is_error() {
                    self.stop_all();
                    return res.content.map(|_| ());
                }
            }
        }
        Ok(())
    }
    pub fn stop_all(&mut self) {
        for (id, (j, control)) in self.task_map.drain() {
            let _ = control.sender().send(ThreadControl::Stop);
            let _ = j.join();
            log::trace!("stop dsl module {}", id)
        }
    }
    fn start_update(&mut self, frame_id: u64) {
        let state_map = self.state_map();
        for (id, (_j, control)) in &mut self.task_map {
            log::trace!("start {} update function at frame {}", id, frame_id);
            let _ = control
                .sender()
                .send(ThreadControl::Start(state_map.clone(), frame_id));
        }
    }
    fn fetch_result(&mut self, lua: &FoolScript, frame_id: u64) -> anyhow::Result<()> {
        let mut is_error = Ok(());
        let mut result_map = Vec::new();
        for (id, (_j, control)) in &mut self.task_map {
            let res = control.receiver().recv()?;
            match res.content {
                Ok(d) => {
                    log::trace!(
                        "dsl module {} update result {:?}, at frame {}",
                        id,
                        d,
                        frame_id
                    );
                    result_map.push((res.id, d))
                }
                Err(err) => {
                    log::trace!(
                        "dsl module {} update failed {:?}, at frame {}",
                        id,
                        err,
                        frame_id
                    );
                    is_error = Err(err);
                    break;
                }
            }
        }
        if is_error.is_err() {
            self.stop_all();
            return is_error;
        }
        let mut modules_lock = self.modules.dsl_mod.modules.write();
        for res in result_map {
            if let Some(m) = modules_lock.get_mut(&res.0) {
                match res.1 {
                    Some(state) => match m.set_state(&lua, state) {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    },
                    None => {}
                }
            }
        }
        Ok(())
    }
    pub fn tick(&mut self, lua: &FoolScript, frame_id: u64) -> anyhow::Result<()> {
        let now = std::time::Instant::now();
        self.start_update(frame_id);
        self.fetch_result(lua, frame_id)?;
        log::trace!("lua elapsed: {:?} ", now.elapsed());
        Ok(())
    }
}
