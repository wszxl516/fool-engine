use mlua::{Lua, Table, Value};
use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct LuaZipModule {
    modules: HashMap<String, String>,
}
impl LuaZipModule {
    pub fn new() -> Self {
        Self {
            modules: Default::default(),
        }
    }
    pub fn init<I, P>(&mut self, lua: &Lua, modules: P) -> mlua::Result<()>
    where
        I: Into<PathBuf> + Sized,
        P: IntoIterator<Item = I>,
    {
        self.add_zip_module(modules);
        let globals = lua.globals();
        let package: Table = globals.get("package")?;
        let searchers: Table = package.get("searchers")?;
        let zip_searcher = {
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
                    Ok((
                        Value::Function(loader),
                        Value::String(lua.create_string(&modname)?),
                    ))
                } else {
                    Ok((
                        Value::Nil,
                        Value::String(lua.create_string("Module not found in zip")?),
                    ))
                }
            })?
        };

        let new_searchers = lua.create_table()?;
        new_searchers.set(1, zip_searcher)?;
        for pair in searchers.clone().sequence_values::<Value>() {
            let i = new_searchers.len()? + 1;
            new_searchers.set(i, pair?)?;
        }

        package.set("searchers", new_searchers)?;
        Ok(())
    }
    fn add_zip_module<P, I>(&mut self, modules: P)
    where
        I: Into<PathBuf> + Sized,
        P: IntoIterator<Item = I>,
    {
        for zip_path in modules {
            let zip_name: PathBuf = zip_path.into();
            match zip_name.with_extension("").file_name() {
                Some(name) => match Self::load_lua_scripts(&zip_name, name.to_string_lossy()) {
                    Ok(modules) => {
                        self.modules.extend(modules);
                    }
                    Err(err) => {
                        log::error!("load modules from {:?} failed: {}", zip_name.to_str(), err)
                    }
                },
                None => continue,
            }
        }
    }
    fn load_lua_scripts(
        zip_path: impl Into<PathBuf>,
        name: impl Into<String>,
    ) -> anyhow::Result<HashMap<String, String>> {
        let zip_path: PathBuf = zip_path.into();
        let file = std::fs::File::open(&zip_path)?;
        let mut module_map = std::collections::HashMap::new();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let name: String = name.into();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let mod_path = Path::new(file.name());
            if file.name().ends_with("init.lua") {
                let mod_name = mod_path
                    .parent()
                    .map(|p| p.to_string_lossy().replace(['/', '\\'], "."))
                    .unwrap_or_default();
                let mod_name = [name.clone(), mod_name].join(".");
                module_map.insert(mod_name.clone(), content.clone());
                log::debug!("load {} from {}", mod_name, zip_path.display())
            }
            if let Some(extension) = mod_path.extension() {
                if extension.to_str() == Some("lua") {
                    let mod_name = Path::new(file.name())
                        .with_extension("")
                        .to_string_lossy()
                        .replace(['/', '\\'], ".");
                    let mod_name = [name.clone(), mod_name].join(".");
                    module_map.insert(mod_name.clone(), content);
                    log::debug!("load {} from {}", mod_name, zip_path.display())
                }
            }
        }
        Ok(module_map)
    }
}
