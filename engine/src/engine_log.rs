use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::Trigger, CompoundPolicy,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::{threshold::ThresholdFilter, Filter, Response},
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
            if self.exists {
                Ok(true)
            } else {
                Ok(false)
            }
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
        let module_name = record.module_path().unwrap_or_default().to_string();
        for mod_name in &self.module_list {
            if module_name.starts_with(mod_name) {
                return Response::Accept;
            }
        }
        Response::Reject
    }
}

const LOG_FILE_COUNT: u32 = 7;
const FORMAT: &str = "{h({d(%+)(utc)} [{f}:{L}:{T}] {l:<6} {M}:{m})}{n}";
pub fn log_init(
    level: LevelFilter,
    console: bool,
    path: &str,
    allow_modules: &[&str],
) -> anyhow::Result<(), SetLoggerError> {
    let path = match Path::new(path).parent() {
        None => "./engine.log",
        Some(p) => {
            if !p.exists() {
                eprintln!(
                    "log path {} not exists use default ./engine.log",
                    p.display()
                );
                "./engine.log"
            } else {
                path
            }
        }
    };
    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(FORMAT)))
        .build();
    let module_filter = ModuleFilter::new(allow_modules);
    let config = Config::builder();
    let config = match console {
        true => config.appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .filter(Box::new(module_filter))
                .build("stdout", Box::new(stdout)),
        ),
        false => {
            let archive_pattern = format!("{}.{{}}.gz", path);
            let roller = FixedWindowRoller::builder()
                .base(0)
                .build(archive_pattern.as_str(), LOG_FILE_COUNT)
                .unwrap();
            let policy = CompoundPolicy::new(
                Box::new(StartupRollTrigger::new(path.to_string())),
                Box::new(roller),
            );
            let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(FORMAT)))
                .build(path, Box::new(policy))
                .unwrap();
            config.appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .filter(Box::new(module_filter))
                    .build("logfile", Box::new(logfile)),
            )
        }
    };
    let root = Root::builder();
    let root = match console {
        true => root.appender("stdout"),
        false => root.appender("logfile"),
    }
    .build(level);
    let config = config.build(root).unwrap();
    log4rs::init_config(config)?;
    Ok(())
}
