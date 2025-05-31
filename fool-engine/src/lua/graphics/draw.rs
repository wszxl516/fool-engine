use mlua::UserData;
pub struct LuaCancvas {}
impl LuaCancvas {
    pub fn new() -> Self {
        LuaCancvas {}
    }
}

impl UserData for LuaCancvas {}
