use clap::Parser;
use fool_script::FoolScript;
use mlua::{Lua, UserData};
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
pub fn register_test(lua: &Lua) -> mlua::Result<mlua::Value> {
    Ok(mlua::Value::UserData(lua.create_userdata(Test)?))
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
    script.register_user_mod("a.b.c", register_test)?;
    script.load_main()?;
    for _ in 0..3 {
        script.run_dsl_update();
    }
    let task = fool_script::thread::AsyncScheduler::new(&script, 2);
    task.run();
    task.wait_all();
    for _ in 0..3 {
        script.run_dsl_update();
    }
    println!("lua main fn return: {}", script.run_fun::<f64>("main", ())?);
    Ok(())
}
