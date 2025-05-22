use crate::map2lua_error;
use crate::resource::{types::LuaFont, types::LuaImage, ResourceManager};
use mlua::{UserData, Value};
use nannou::text::{rt::Point, Font, Scale};
use std::sync::{Arc, Mutex};
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
        methods.add_method(
            "measure_text",
            |lua, _this, (text, font, font_size): (String, Value, u32)| {
                let font = match font {
                    mlua::Value::UserData(ud) => ud.borrow::<LuaFont>()?,
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: font.type_name(),
                            to: "LuaFont".into(),
                            message: Some("Expected LuaFont as UserData".into()),
                        });
                    }
                };
                let size = measure_text(&font.graphics, &text, font_size as f32);
                let table = lua.create_table()?;
                table.set("w", size.0 * 1.3)?;
                table.set("h", size.1 * 1.3)?;
                Ok(table)
            },
        );
        methods.add_method("load_texture", |lua, this, path: String| {
            let mut mgr = map2lua_error!(this.res_mgr.lock(), "ResourceManager Error:")?;
            let texture = map2lua_error!(mgr.get_texture(path), "lua load_texture Error")?;
            Ok(lua.create_any_userdata(texture))
        });
        methods.add_method("load_font", |lua, this, path: String| {
            let mut mgr = map2lua_error!(this.res_mgr.lock(), "ResourceManager Error")?;
            let texture = map2lua_error!(mgr.get_font(path), "lua load_font Error: ")?;
            Ok(lua.create_any_userdata(texture))
        });

        methods.add_method("load_image", |lua, this, path: String| {
            let mut mgr = map2lua_error!(this.res_mgr.lock(), "ResourceManager Error")?;
            let image = map2lua_error!(mgr.get_image(path), "lua load_image Error")?.clone();
            Ok(lua.create_any_userdata(LuaImage { image }))
        });
    }
}

pub fn measure_text(font: &Font, text: &str, font_size: f32) -> (f32, f32) {
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);
    let height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap).ceil();

    let width = font
        .layout(text, scale, Point { x: 0.0, y: 0.0 })
        .fold(0.0, |acc: f32, g| {
            if let Some(bb) = g.pixel_bounding_box() {
                acc.max(bb.max.x as f32)
            } else {
                acc
            }
        });

    (width, height)
}
