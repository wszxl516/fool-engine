use crate::map2lua_error;
use crate::resource::{types::LuaImage, ResourceManager};
use mlua::UserData;
use parking_lot::Mutex;
use std::sync::Arc;
pub struct LuaResourceManager {
    res_mgr: Arc<Mutex<ResourceManager>>,
}
impl LuaResourceManager {
    pub fn new(res_mgr: Arc<Mutex<ResourceManager>>) -> Self {
        Self { res_mgr }
    }
}
impl UserData for LuaResourceManager {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("load_image", |lua, this, path: String| {
            let mut mgr = this.res_mgr.lock();
            let image = map2lua_error!(mgr.get_image(path), "lua load_image Error")?.clone();
            Ok(lua.create_any_userdata(LuaImage { image }))
        });
    }
}
