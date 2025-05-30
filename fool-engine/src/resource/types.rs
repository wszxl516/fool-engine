use image::DynamicImage;
use mlua::UserData;

#[derive(Clone)]
pub struct LuaImage {
    pub image: DynamicImage,
}
impl UserData for LuaImage {}
