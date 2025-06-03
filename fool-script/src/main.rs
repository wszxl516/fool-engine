use clap::Parser;
use fool_script::FoolScript;
use fool_script::user_mod_constructor;
use mlua::UserData;
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
struct Test;
impl UserData for Test {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("test", |_, ()| {
            println!("test!");
            Ok(())
        });
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let level = if args.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Warn
    };
    env_logger::Builder::new().filter_level(level).init();
    let mut script = FoolScript::new("./fool-script")?;
    let mut modules = HashMap::<String, Vec<u8>>::new();
    modules.insert(
        "mem_module.lua".to_owned(),
        r#"
    return {
    main = function ()
        print("mem_module")
    end
    }
    "#
        .as_bytes()
        .to_vec(),
    );
    script.setup(&modules)?;
    script.register_user_mod("a.b.c", user_mod_constructor!(Test))?;
    script.load_main()?;
    let task = fool_script::thread::AsyncScheduler::new(&script, 2);
    for _ in 0..10 {
        task.run();
        task.wait_all()?;
    }
    println!("lua main fn return: {}", script.run_fun::<f64>("main", ())?);
    Ok(())
}
