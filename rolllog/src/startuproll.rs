use log4rs::{
    append::rolling_file::policy::compound::trigger::Trigger,
    filter::{Filter, Response},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::HashSet, path::Path};

#[derive(Debug, Default)]
pub struct StartupRollTrigger {
    triggered: AtomicBool,
    exists: bool,
}

impl StartupRollTrigger {
    pub fn new(log_path: String) -> Self {
        let path = Path::new(&log_path);
        let exists = if path.exists() { true } else { false };
        StartupRollTrigger {
            triggered: AtomicBool::new(false),
            exists,
        }
    }
}

impl Trigger for StartupRollTrigger {
    fn is_pre_process(&self) -> bool {
        true
    }

    fn trigger(&self, _file: &log4rs::append::rolling_file::LogFile) -> anyhow::Result<bool> {
        if !self.triggered.load(Ordering::Relaxed) {
            self.triggered.store(true, Ordering::Relaxed);
            if self.exists { Ok(true) } else { Ok(false) }
        } else {
            Ok(false)
        }
    }
}
#[derive(Debug, Default)]
pub struct ModuleFilter {
    module_list: HashSet<String>,
}
impl ModuleFilter {
    pub fn new(module_name: &[&str]) -> Self {
        let mut module_list = HashSet::new();
        for m in module_name {
            module_list.insert(m.to_string());
        }
        Self { module_list }
    }
}
impl Filter for ModuleFilter {
    fn filter(&self, record: &log::Record) -> log4rs::filter::Response {
        if self.module_list.is_empty() {
            return Response::Reject;
        }
        let module_name = record.module_path().unwrap_or_default().to_string();
        for mod_name in &self.module_list {
            if module_name.starts_with(mod_name) {
                return Response::Accept;
            }
        }
        Response::Reject
    }
}
