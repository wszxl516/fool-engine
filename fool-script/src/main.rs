use clap::Parser;
use fool_script::FoolScript;
use fool_script::user_mod_constructor;
use mlua::UserData;
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
use fool_resource::{Fallback, Resource, SharedData};
use std::io::Read;
#[derive(Debug, Clone)]
struct FileFallBack {
    pub asset_path: std::path::PathBuf,
}
impl Fallback for FileFallBack {
    type K = String;
    type V = SharedData;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let full_path = self.asset_path.join(key);
        println!("{:?}", full_path);
        let mut fd = std::fs::File::open(full_path)?;
        let mut buffer = Vec::new();
        fd.read_to_end(&mut buffer)?;
        Ok(SharedData::from(buffer))
    }
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let level = if args.verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Warn
    };
    env_logger::Builder::new().filter_level(level).init();
    let fbk = FileFallBack {
        asset_path: "/data/works/game_engine/fool-script".into(),
    };
    let res = Resource::<String, SharedData>::empty();
    res.set_fall_back(fbk);
    res.load(
        "mem_module.lua".to_owned(),
        fool_resource::SharedData::from_vec(
            r#"
    return {
    main = function ()
        print("mem_module")
    end
    }
    "#
            .as_bytes()
            .to_vec(),
        ),
    );
    let mut script = FoolScript::new(res)?;
    script.setup()?;
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
