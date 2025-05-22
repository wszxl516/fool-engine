use crate::resource::ResourceManager;
use mlua::{Lua, Table, Value};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, path::Path};
#[derive(Debug, Clone)]
pub struct MemoryModule {
    modules: HashMap<String, String>,
}
impl MemoryModule {
    pub fn new() -> Self {
        Self {
            modules: Default::default(),
        }
    }
    pub fn init(&mut self, lua: &Lua, res_mgr: Arc<Mutex<ResourceManager>>) -> mlua::Result<()> {
        self.load_lua_modules(res_mgr)?;
        let globals = lua.globals();
        let package: Table = globals.get("package")?;
        let searchers: Table = package.get("searchers")?;
        let memory_searcher = {
            let module_map = self.modules.clone();
            lua.create_function(move |lua, modname: String| {
                if let Some(script) = module_map.get(&modname) {
                    let script = script.to_owned();
                    let modname_cloned = modname.clone();
                    let loader = lua.create_function(move |lua, _: ()| {
                        lua.load(script.clone())
                            .set_name(&modname_cloned)
                            .eval::<Value>()
                    })?;
                    log::debug!("lua module {} loaded!", modname);
                    Ok((
                        Value::Function(loader),
                        Value::String(lua.create_string(&modname)?),
                    ))
                } else {
                    log::error!("module {} not found!", modname);
                    Ok((
                        Value::Nil,
                        Value::String(lua.create_string("Module not found in zip")?),
                    ))
                }
            })?
        };

        let new_searchers = lua.create_table()?;
        new_searchers.set(1, memory_searcher)?;
        for pair in searchers.clone().sequence_values::<Value>() {
            let i = new_searchers.len()? + 1;
            new_searchers.set(i, pair?)?;
        }

        package.set("searchers", new_searchers)?;
        Ok(())
    }
    fn load_lua_modules(&mut self, res_mgr: Arc<Mutex<ResourceManager>>) -> mlua::Result<()> {
        let lock = res_mgr.lock().unwrap();
        for (name, content) in lock.memory_resource.all_resource() {
            let mod_path = Path::new(&name);
            if let Some(extension) = mod_path.extension() {
                if extension.to_str() == Some("lua") || mod_path.ends_with("init.lua") {
                    let mod_name = mod_path
                        .with_extension("")
                        .to_string_lossy()
                        .replace(['/', '\\'], ".");
                    self.modules.insert(
                        mod_name.clone(),
                        String::from_utf8(content.clone()).unwrap(),
                    );
                    log::debug!("load {} from ResourceManager!", mod_name)
                }
            }
        }
        Ok(())
    }
}
