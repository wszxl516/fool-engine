use egui::epaint::TextureHandle;
use mlua::UserData;
#[derive(Clone)]
pub struct LuaImage {
    pub image: TextureHandle,
}
impl UserData for LuaImage {}
