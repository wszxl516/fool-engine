use egui::Context;
use mlua::UserData;
use nannou::{
    image::{DynamicImage, GenericImageView},
    text::Font,
    wgpu::{DeviceQueuePair, Texture},
};
use nannou_egui::egui::{
    epaint::textures::TextureOptions, ColorImage, FontData, FontDefinitions, FontFamily,
    TextureHandle,
};
use std::sync::Arc;
#[derive(Clone)]
pub struct LuaFont {
    pub ui: FontDefinitions,
    pub graphics: Font,
}
impl LuaFont {
    pub fn from_bytes(buffer: Vec<u8>, name: &String) -> anyhow::Result<Self> {
        let mut ui_fonts = FontDefinitions::empty();
        ui_fonts
            .font_data
            .insert(name.clone(), FontData::from_owned(buffer.clone()));
        if let Some(family) = ui_fonts.families.get_mut(&FontFamily::Proportional) {
            family.insert(0, name.clone());
        }
        if let Some(family) = ui_fonts.families.get_mut(&FontFamily::Monospace) {
            family.insert(0, name.clone());
        }
        let graphics_font = Font::from_bytes(buffer.clone())
            .map_err(|e| anyhow::anyhow!(format!("graphics_font load failed: {}", e)))?;
        Ok(Self {
            ui: ui_fonts,
            graphics: graphics_font,
        })
    }
}
impl UserData for LuaFont {}
#[derive(Clone)]
pub struct LuaTexture {
    pub ui: TextureHandle,
    pub graphics: Texture,
}

impl LuaTexture {
    pub fn from_image(
        name: &String,
        img: &DynamicImage,
        dev: &Arc<DeviceQueuePair>,
        ctx: Context,
    ) -> anyhow::Result<Self> {
        let usage = nannou::wgpu::TextureBuilder::default_image_texture_usage();
        let graphics_texture =
            nannou::wgpu::Texture::load_from_image(dev.device(), dev.queue(), usage, img);
        let rgba_image = img.to_rgba8();
        let (width, height) = (rgba_image.width() as usize, rgba_image.height() as usize);
        let pixels: Vec<egui::Color32> = img
            .pixels()
            .map(|p| {
                egui::Color32::from_rgba_premultiplied(p.2 .0[0], p.2 .0[1], p.2 .0[2], p.2 .0[3])
            })
            .collect();
        let color_image = ColorImage {
            size: [width as usize, height as usize],
            pixels,
        };

        let ui_texture = ctx.load_texture(name, color_image, TextureOptions::default());
        Ok(Self {
            ui: ui_texture,
            graphics: graphics_texture,
        })
    }
}

impl UserData for LuaTexture {}

#[derive(Clone)]
pub struct LuaImage {
    pub image: DynamicImage,
}
impl UserData for LuaImage {}
