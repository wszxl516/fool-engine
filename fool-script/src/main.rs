use fool_script::FoolScript;
use fool_script::user_mod_constructor;
use mlua::UserData;
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
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();
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
    let mut task = fool_script::thread::AsyncScheduler::new(script.modules.clone());
    task.init()?;
    for n in 0..100 {
        task.fetch_result(&script, n)?;
        println!("lua main fn return: {}", script.run_fun::<f64>("main", ())?);
        task.start_update(n);
    }
    Ok(())
}
