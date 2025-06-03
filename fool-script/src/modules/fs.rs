use mlua::{Function, Lua, Value};
use std::path::PathBuf;
#[cfg(feature = "debug")]
pub fn fs_loader(lua: &Lua, script_path: PathBuf) -> mlua::Result<Function> {
    let fs_searcher = lua.create_function(move |lua, modname: String| {
        let relative_path = modname.replace('.', "/") + ".lua";
        let full_path = script_path.join(&relative_path);
        log::trace!("lua module full_path {} !", full_path.display());
        match std::fs::read_to_string(&full_path) {
            Ok(script) => {
                let modname_cloned = modname.clone();
                let loader = lua.create_function(move |lua, _: ()| {
                    let value = lua
                        .load(&script)
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
            }
            Err(e) => {
                log::error!("module {} not found: {}!", modname, e);
                Ok((
                    Value::Nil,
                    Value::String(
                        lua.create_string(format!("Could not load module '{}': {}", modname, e))?,
                    ),
                ))
            }
        }
    })?;
    Ok(fs_searcher)
}

#[cfg(not(feature = "debug"))]
pub fn fs_loader(lua: &Lua, _script_path: PathBuf) -> mlua::Result<Function> {
    lua.create_function(|lua, modname: String| {
        log::debug!("load module {} from fs is not supported!", modname);
        Ok((
            Value::Nil,
            Value::String(lua.create_string("load module from fs is not supported!")?),
        ))
    })
}
