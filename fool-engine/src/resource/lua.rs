#![allow(dead_code)]
use crate::event::EngineEventLoop;
use crate::map2lua_error;
use crate::resource::ResourceManager;
use mlua::UserData;
pub struct LuaResourceManager {
    res_mgr: ResourceManager,
    event_loop: EngineEventLoop,
}
impl LuaResourceManager {
    pub fn new(res_mgr: ResourceManager, event_loop: EngineEventLoop) -> Self {
        Self {
            res_mgr,
            event_loop,
        }
    }
}
impl UserData for LuaResourceManager {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("preload_ui_texture", |_lua, this, path: String| {
            map2lua_error!(this.res_mgr.preload_ui_texture(path), "preload_ui_texture")?;
            Ok(())
        });
    }
}
