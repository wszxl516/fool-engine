use crate::map2anyhow_error;
use egui::{FontData, FontDefinitions};
use image::DynamicImage;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
pub mod lua;
pub mod types;
use egui::epaint::TextureHandle;
#[cfg(not(feature = "debug"))]
use packtool::MemResource;
use winit::{
    event_loop::ActiveEventLoop,
    window::{CustomCursor, Icon},
};
pub enum Resource {
    Image(DynamicImage),
}
pub struct ResourceManager {
    resources: HashMap<String, Resource>,
    #[cfg(not(feature = "debug"))]
    pub memory_resource: MemResource,
    #[cfg(feature = "debug")]
    assets_path: PathBuf,
    pub ui_font: FontDefinitions,
    pub window_cursor: HashMap<String, CustomCursor>,
    pub window_icon: HashMap<String, Icon>,
    pub ui_texture: HashMap<String, TextureHandle>,
}

pub fn resource_path() -> anyhow::Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    #[cfg(feature = "debug")]
    const RESOURCES_PATH: &str = "assets";
    #[cfg(not(feature = "debug"))]
    const RESOURCES_PATH: &str = "resources";
    let path = map2anyhow_error!(
        find_folder::Search::ParentsThenKids(5, 3)
            .of(exe_path
                .parent()
                .expect("executable has no parent directory to search")
                .into())
            .for_folder(RESOURCES_PATH),
        "get resource_path failed"
    )?;
    Ok(path)
}
impl ResourceManager {
    pub fn new() -> anyhow::Result<Self> {
        let assets_path = resource_path()?;
        #[cfg(not(feature = "debug"))]
        let resource_pack =
            packtool::ResourcePackage::from_pak(assets_path.join("assets.pak"))?.unpack2memory()?;
        Ok(Self {
            resources: HashMap::new(),
            #[cfg(not(feature = "debug"))]
            memory_resource: resource_pack,
            #[cfg(feature = "debug")]
            assets_path,
            ui_font: FontDefinitions::empty(),
            window_cursor: Default::default(),
            window_icon: Default::default(),
            ui_texture: Default::default(),
        })
    }
    #[cfg(not(feature = "debug"))]
    pub fn load_bytes_from_memory(&self, path: &String) -> &[u8] {
        self.memory_resource
            .get(path)
            .expect(&format!("resource {} not found!", path))
    }
    #[cfg(not(feature = "debug"))]
    pub fn all_memory_resource(&self) -> &HashMap<String, Vec<u8>> {
        &self.memory_resource
    }
    pub fn load_image(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        #[cfg(feature = "debug")]
        {
            let img_path = self.assets_path.join(&path);
            let img = image::open(&img_path)?;
            log::debug!("load imge {} from disk!", img_path.display());
            self.resources
                .insert(path.to_string_lossy().to_string(), Resource::Image(img));
        }
        #[cfg(not(feature = "debug"))]
        {
            let img = map2anyhow_error!(
                image::load_from_memory(
                    self.load_bytes_from_memory(&path.to_string_lossy().to_string())
                ),
                "load_image failed"
            )?;
            log::debug!("load imge {} from memory!", &path.display());
            self.resources
                .insert(path.to_string_lossy().to_string(), Resource::Image(img));
        }
        Ok(())
    }
    pub fn get_image(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<&DynamicImage> {
        let path: PathBuf = path.into();
        if !self
            .resources
            .contains_key(&path.to_string_lossy().to_string())
        {
            self.load_image(&path)?;
        }
        let id = path.to_string_lossy().to_string();
        self.resources
            .get(&id)
            .and_then(|res| match res {
                Resource::Image(image) => Some(image),
            })
            .ok_or_else(|| anyhow::anyhow!("resource is not a image or not found!"))
    }
    pub fn load_ui_font(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let id = path.to_string_lossy().to_string();
        #[cfg(feature = "debug")]
        {
            let font_path = self.assets_path.join(&path);
            let font_data = std::fs::read(&font_path)?;
            log::debug!("load font {} from disk!", font_path.display());
            let font_data = Arc::new(FontData::from_owned(font_data));
            self.ui_font.font_data.insert(id.clone(), font_data);
        }
        #[cfg(not(feature = "debug"))]
        {
            log::debug!("load imge {} from memory!", &path.display());
            let font_data = self.load_bytes_from_memory(&path.to_string_lossy().to_string());
            let font_data = Arc::new(FontData::from_owned(font_data.into()));
            self.ui_font.font_data.insert(id.clone(), font_data);
        };

        self.ui_font
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, id.clone());
        self.ui_font
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push(id);
        Ok(())
    }
    pub fn load_cursor(
        &mut self,
        name: &String,
        event_loop: &ActiveEventLoop,
    ) -> anyhow::Result<()> {
        let img = self.get_image(&name)?;
        match create_cursor(event_loop, img) {
            Ok(c) => {
                self.window_cursor.insert(name.clone(), c);
                log::debug!("cursor {} loaded!", name);
            }
            Err(err) => {
                log::error!("load cursor {} failed: {}", name, err);
            }
        }

        Ok(())
    }
    pub fn get_cursor(&self, path: &String) -> Option<&CustomCursor> {
        self.window_cursor.get(path)
    }
    pub fn load_window_icon(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let img = self.get_image(&path)?;
        let width = img.width();
        let height = img.height();
        let rgba = img.as_rgba8().ok_or(anyhow::anyhow!(
            "convert {} to rgba8 failed!",
            path.display()
        ))?;
        let icon = Icon::from_rgba(rgba.clone().into_vec(), width, height)?;
        self.window_icon
            .insert(path.to_string_lossy().to_string(), icon);
        Ok(())
    }
    pub fn get_window_icon(&mut self, path: &String) -> anyhow::Result<&Icon> {
        if !self.window_icon.contains_key(path) {
            self.load_window_icon(path)?;
        }
        self.window_icon
            .get(path)
            .ok_or(anyhow::anyhow!("window_icon {} not found!", path))
    }
    pub fn load_ui_texture(
        &mut self,
        path: impl Into<PathBuf>,
        ctx: &egui::Context,
    ) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let id = path.to_string_lossy().to_string();
        let img = self.get_image(&path)?;
        let texture = texture_from_image(&id, img, ctx)?;
        self.ui_texture.insert(id, texture);
        Ok(())
    }
    pub fn get_ui_texture(&mut self, path: &String) -> anyhow::Result<&TextureHandle> {
        self.ui_texture
            .get(path)
            .ok_or_else(|| anyhow::anyhow!("resource is not a texture or not found!"))
    }
}

pub fn create_cursor(
    event_loop: &ActiveEventLoop,
    img: &DynamicImage,
) -> anyhow::Result<CustomCursor> {
    let width = img.width() as u16;
    let height = img.height() as u16;
    let rgba = img.as_rgba8().cloned().unwrap().into_vec();
    let cursor = CustomCursor::from_rgba(rgba, width, height, width / 2, height / 2)?;
    Ok(event_loop.create_custom_cursor(cursor))
}

pub fn texture_from_image(
    name: &String,
    img: &image::DynamicImage,
    ctx: &egui::Context,
) -> anyhow::Result<TextureHandle> {
    use egui::ColorImage;
    use egui::TextureOptions;
    use image::GenericImageView;
    let rgba_image = img.to_rgba8();
    let (width, height) = (rgba_image.width() as usize, rgba_image.height() as usize);
    let pixels: Vec<egui::Color32> = img
        .pixels()
        .map(|p| egui::Color32::from_rgba_premultiplied(p.2 .0[0], p.2 .0[1], p.2 .0[2], p.2 .0[3]))
        .collect();
    let color_image = ColorImage {
        size: [width as usize, height as usize],
        pixels,
    };

    let ui_texture = ctx.load_texture(name, color_image, TextureOptions::default());
    // let t:egui::ImageSource = (&ui_texture).into();
    // let img: egui::Image = t.into();
    Ok(ui_texture)
}
