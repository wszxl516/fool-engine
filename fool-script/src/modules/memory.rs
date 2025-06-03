use fool_resource::{Resource, SharedData};
use mlua::{Function, Lua, Value};
use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Debug, path::Path};
#[derive(Debug, Clone)]
pub struct MemoryModule {
    modules: Arc<RwLock<HashMap<String, Arc<String>>>>,
    resource: Resource<String, SharedData>,
}
impl MemoryModule {
    pub fn new(resource: Resource<String, SharedData>) -> Self {
        Self {
            modules: Default::default(),
            resource,
        }
    }
    pub fn get_or_insert(&self, modname: &String) -> mlua::Result<Arc<String>> {
        let module = { self.modules.read().get(modname).cloned() };
        match module {
            Some(content) => Ok(content),
            None => {
                let file_path = modname.replace('.', "/") + ".lua";
                match self.resource.get(&file_path) {
                    Ok(content) => {
                        let script = Arc::new(content.to_string().map_err(|err| {
                            mlua::Error::RuntimeError(format!(
                                "{} not a Correct lua script: {}",
                                file_path, err
                            ))
                        })?);
                        self.modules.write().insert(modname.clone(), script.clone());
                        log::trace!("module {} is load from from {}!", modname, file_path);
                        Ok(script)
                    }
                    Err(err) => {
                        log::trace!("module {} load from {} failed: {}", modname, file_path, err);
                        Err(mlua::Error::RuntimeError(format!(
                            "script {} not found: {}",
                            file_path, err
                        )))
                    }
                }
            }
        }
    }
    pub fn init(&self, lua: &Lua) -> mlua::Result<Function> {
        let resource = self.clone();
        let memory_searcher = {
            lua.create_function(move |lua, modname: String| {
                if let Ok(script) = resource.get_or_insert(&modname) {
                    let script = script.to_owned();
                    let modname_cloned = modname.clone();
                    let loader = lua.create_function(move |lua, _: ()| {
                        let value = lua
                            .load(script.as_ref())
                            .set_name(&modname_cloned)
                            .eval::<Value>()?;
                        let value = crate::utils::set_module_name(value, &modname_cloned, lua)?;
                        Ok(value)
                    })?;
                    log::trace!("lua module {} found!", modname);
                    Ok((
                        Value::Function(loader),
                        Value::String(lua.create_string(&modname)?),
                    ))
                } else {
                    log::trace!("lua module {} not found!", modname);
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
        Ok(memory_searcher)
    }
}
