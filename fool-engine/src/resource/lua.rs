#![allow(dead_code)]
use crate::event::EngineEventLoop;
use crate::resource::ResourceManager;
use mlua::UserData;
use parking_lot::Mutex;
use std::sync::Arc;
pub struct LuaResourceManager {
    res_mgr: Arc<Mutex<ResourceManager>>,
    event_loop: EngineEventLoop,
}
impl LuaResourceManager {
    pub fn new(res_mgr: Arc<Mutex<ResourceManager>>, event_loop: EngineEventLoop) -> Self {
        Self {
            res_mgr,
            event_loop,
        }
    }
}
impl UserData for LuaResourceManager {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("load_texture", |_lua, this, path: String| {
            this.event_loop.load_ui_texture(path);
            Ok(())
        });
    }
}
