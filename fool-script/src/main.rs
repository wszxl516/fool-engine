use clap::Parser;
use std::collections::HashMap;
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// off, error, warn, info, debug, trace,
    #[arg(short, long, default_value = None)]
    log_level: Option<String>,
    /// script
    #[arg(short, long, default_value = "./main.lua")]
    script: String,
    /// The log is output to the console
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let level = if args.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Warn
    };
    env_logger::Builder::new().filter_level(level).init();
    let mut script = fool_script::LuaBindings::new("./fool-script")?;
    let modules = HashMap::<String, Vec<u8>>::new();
    script.setup(&modules)?;
    script.load_main()?;
    for _ in 0..10 {
        script.run_dsl_update();
    }
    println!(
        "lua main fn return: {}",
        script.run_main_fn::<f64>("main", ())?
    );
    Ok(())
}
