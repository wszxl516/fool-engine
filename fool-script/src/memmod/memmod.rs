use mlua::{Lua, Table, Value};
use std::{collections::HashMap, fmt::Debug, path::Path};
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
    pub fn init<K, V, M>(&mut self, lua: &Lua, modules: M) -> mlua::Result<()>
    where
        K: Into<String>,
        V: AsRef<[u8]> + Clone,
        M: IntoIterator<Item = (K, V)>,
    {
        self.load_lua_module(modules)?;
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
                    log::debug!("lua module {} found!", modname);
                    Ok((
                        Value::Function(loader),
                        Value::String(lua.create_string(&modname)?),
                    ))
                } else {
                    log::error!("module {} not found!", modname);
                    Ok((
                        Value::Nil,
                        Value::String(lua.create_string(format!(
                            "Module {} not found in MemoryModule Loader!",
                            modname
                        ))?),
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
    fn load_lua_module<K, V, M>(&mut self, modules: M) -> mlua::Result<()>
    where
        K: Into<String>,
        V: AsRef<[u8]> + Clone,
        M: IntoIterator<Item = (K, V)>,
    {
        for (name, content) in modules.into_iter() {
            let name: String = name.into();
            let mod_path = Path::new(&name);
            let extension = mod_path.extension().and_then(|e| e.to_str());
            if extension == Some("lua") || extension == Some("init.lua") {
                let mod_name = if mod_path.file_name().and_then(|f| f.to_str()) == Some("init.lua")
                {
                    mod_path
                        .parent()
                        .map(|p| p.to_string_lossy().replace(['/', '\\'], "."))
                        .unwrap_or_default()
                } else {
                    mod_path
                        .with_extension("")
                        .to_string_lossy()
                        .replace(['/', '\\'], ".")
                };
                if let Ok(content) = std::str::from_utf8(content.as_ref()).map(|s| s.to_string()) {
                    self.modules.insert(mod_name.clone(), content);
                    log::debug!("lua module {} ({}) loaded!", name, mod_name);
                } else {
                    log::debug!("lua module {} ({}) failed!", name, mod_name);
                }
            }
        }
        Ok(())
    }
}
