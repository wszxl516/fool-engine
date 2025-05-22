mod startuproll;
use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::policy::compound::{CompoundPolicy, roll::fixed_window::FixedWindowRoller},
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use startuproll::{ModuleFilter, StartupRollTrigger};

const LOG_FILE_COUNT: u32 = 7;
const FORMAT: &str = "{h({d(%+)(utc)} [{f}:{L}:{T}] {l:<6} {M} {m})}{n}";
pub fn log_init(
    level: LevelFilter,
    console: bool,
    path: &str,
    allow_modules: &[&str],
) -> anyhow::Result<(), SetLoggerError> {
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
