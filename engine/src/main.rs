#![cfg_attr(not(feature = "debug"), windows_subsystem = "windows")]
use clap::Parser;
use engine::init_engine;
use log::LevelFilter;
use std::str::FromStr;
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// off, error, warn, info, debug, trace,
    #[arg(short, long, default_value = "info")]
    log_level: String,
    /// log to file
    #[arg(short, long, default_value = "./log.log")]
    file_log: String,
    /// The log is output to the console
    #[arg(short, long, default_value_t = true)]
    verbose: bool,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let level =
        LevelFilter::from_str(args.log_level.as_str()).unwrap_or_else(|_| LevelFilter::Info);
    rolllog::log_init(level, args.verbose, &args.file_log, &["engine"])?;
    init_engine()
}
