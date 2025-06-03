mod macros;
mod modules;
pub mod thread;
mod utils;
use mlua::{AsChunk, FromLuaMulti, Function, IntoLuaMulti, Lua, LuaOptions, StdLib, Table};
use modules::{DSLModule, MemoryModule, UserMod, UserModConstructor, fs_loader, stdlib};
use std::collections::HashMap;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct FoolScript {
    pub lua: Lua,
    script_path: PathBuf,
    mem_mod: MemoryModule,
    dsl_mod: DSLModule,
    user_mod: UserMod,
}

impl FoolScript {
    pub fn new(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let lua = map2anyhow_error!(
            Lua::new_with(
                StdLib::COROUTINE
                    | StdLib::STRING
                    | StdLib::TABLE
                    | StdLib::MATH
                    | StdLib::PACKAGE
                    | StdLib::UTF8,
                LuaOptions::default(),
            ),
            "init lua failed"
        )?;
        Ok(Self {
            lua: lua.clone(),
            script_path: path.into(),
            mem_mod: MemoryModule::new(),
            dsl_mod: DSLModule::new(),
            user_mod: UserMod::new(),
        })
    }
    pub fn setup<K, V, M>(&mut self, modules: M) -> anyhow::Result<()>
    where
        K: Into<String>,
        V: AsRef<[u8]> + Clone,
        M: IntoIterator<Item = (K, V)>,
    {
        let mem_loader = map2anyhow_error!(
            self.mem_mod.init(&self.lua, modules),
            "setup mem loader failed"
        )?;
        let fs_loader = map2anyhow_error!(
            fs_loader(&self.lua, self.script_path.clone()),
            "setup fs loader failed"
        )?;
        let user_loader =
            map2anyhow_error!(self.user_mod.init(&self.lua), "setup fs loader failed")?;
        self.register_module_searcher(&[mem_loader, fs_loader, user_loader])?;
        map2anyhow_error!(self.dsl_mod.init(&self.lua), "setup_dsl_lua failed: {}")?;
        map2anyhow_error!(stdlib::init_stdlib(&self.lua), "init_stdlib failed")?;
        stdlib::enable_debug(&self.lua)?;
        Ok(())
    }
    fn register_module_searcher(&self, searchers: &[Function]) -> anyhow::Result<()> {
        let package: Table =
            map2anyhow_error!(self.lua.globals().get("package"), "get lua module package ")?;
        let new_searchers = map2anyhow_error!(self.lua.create_table(), "lua create table")?;
        for (index, searcher) in searchers.iter().enumerate() {
            map2anyhow_error!(
                new_searchers.set(index + 1, searcher.clone()),
                "lua set searcher!"
            )?;
        }
        map2anyhow_error!(
            package.set("searchers", new_searchers),
            "set package.searchers"
        )?;
        Ok(())
    }
}

impl FoolScript {
    pub fn run<'a>(&self, code: impl AsChunk<'a>, name: impl Into<String>) -> anyhow::Result<()> {
        map2anyhow_error!(
            self.lua.load(code).set_name(name).exec(),
            "run lua code failed"
        )?;
        Ok(())
    }

    pub fn load_main(&self) -> anyhow::Result<()> {
        #[cfg(feature = "debug")]
        {
            use std::io::Read;
            let script = self.script_path.join("main.lua");
            let mut fd = map2anyhow_error!(std::fs::File::open(&script), "load main.lua failed")?;
            let mut script = String::new();
            fd.read_to_string(&mut script)?;
            map2anyhow_error!(self.lua.load(&script).exec(), "run main.lua failed")
        }
        #[cfg(not(feature = "debug"))]
        map2anyhow_error!(
            self.lua.load("require(\"main\")").exec(),
            "run require(\"main\") failed"
        )
    }
    pub fn run_fun<R: FromLuaMulti>(
        &self,
        name: &str,
        args: impl IntoLuaMulti,
    ) -> anyhow::Result<R> {
        let func: Function =
            map2anyhow_error!(self.lua.globals().get(name), "get main func failed:")?;
        map2anyhow_error!(func.call::<R>(args), "run main func")
    }
    pub fn run_module_fun<R: FromLuaMulti>(
        &self,
        mod_name: &String,
        func_name: &String,
        args: impl IntoLuaMulti,
    ) -> anyhow::Result<R> {
        let globals = self.lua.globals();
        let require: mlua::Function =
            map2anyhow_error!(globals.get("require"), "get require failed:")?;
        let module: mlua::Table = map2anyhow_error!(
            require.call(mod_name.as_str()),
            format!("require module {} failed:", mod_name)
        )?;
        let func: mlua::Function = module.get(func_name.as_str())?;
        Ok(func.call::<R>(args)?)
    }
}
impl FoolScript {
    pub fn register_user_mod(
        &self,
        mod_path: &str,
        module: impl UserModConstructor + 'static,
    ) -> anyhow::Result<()> {
        self.user_mod.register(mod_path, module);
        Ok(())
    }
}

impl FoolScript {
    // for multi thread
    pub fn setup_modules_from(other: &Self) -> anyhow::Result<Self> {
        let mut fs = Self::new(other.script_path.clone())?;
        fs.mem_mod = other.mem_mod.clone();
        fs.dsl_mod = other.dsl_mod.clone();
        fs.user_mod = other.user_mod.clone();
        fs.setup(HashMap::<String, Vec<u8>>::default())?;
        Ok(fs)
    }
}
