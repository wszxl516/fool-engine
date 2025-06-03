use crate::map2anyhow_error;
use egui::{FontData, FontDefinitions};
use fool_graphics::canvas::SceneGraph;
use image::DynamicImage;
use std::{path::PathBuf, sync::Arc};
mod fallback;
pub mod lua;
pub mod types;
pub mod utils;
use egui::epaint::TextureHandle;
pub use fool_graphics::canvas::FontManager;
pub use fool_resource::{Resource, SharedData};
use parking_lot::RwLock;
pub use utils::{create_cursor, texture_from_image};
use winit::{
    event_loop::ActiveEventLoop,
    window::{CustomCursor, Icon},
};
#[derive(Clone)]
pub struct ResourceManager {
    pub assets_path: PathBuf,
    pub raw_resource: Resource<String, SharedData>,
    image: Resource<String, Arc<DynamicImage>>,
    pub egui_font: Arc<RwLock<FontDefinitions>>,
    pub egui_texture: Resource<String, TextureHandle>,
    pub window_cursor: Resource<String, Arc<CustomCursor>>,
    pub window_icon: Resource<String, Arc<Icon>>,
    pub scene: Arc<RwLock<SceneGraph>>,
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
        format!("get resource_path {} failed", RESOURCES_PATH)
    )?;
    Ok(path)
}
impl ResourceManager {
    pub fn new() -> anyhow::Result<Self> {
        let assets_path = resource_path()?;
        #[cfg(feature = "debug")]
        let raw = {
            log::debug!(
                "init resource manager assets_path from {}",
                assets_path.display()
            );
            let fs_fallback = fallback::FSFallBack {
                asset_path: assets_path.clone(),
            };
            Resource::from_fallback(fs_fallback)
        };

        #[cfg(not(feature = "debug"))]
        let raw = {
            let assets_path = assets_path.join("assets.pak");
            log::debug!(
                "init resource manager assets_path from {}",
                assets_path.display()
            );
            let resource_pack =
                packtool::ResourcePackage::from_pak(assets_path)?.unpack2memory()?;
            let raw = Resource::empty();
            raw.load_from_map(resource_pack);
            raw
        };

        Ok(Self {
            image: Default::default(),
            raw_resource: raw,
            assets_path,
            egui_font: Arc::new(RwLock::new(FontDefinitions::empty())),
            window_cursor: Default::default(),
            window_icon: Default::default(),
            egui_texture: Default::default(),
            scene: Arc::new(RwLock::new(SceneGraph::default())),
        })
    }
    pub fn load_bytes_from_memory(&self, path: &String) -> SharedData {
        self.raw_resource
            .get(path)
            .expect(&format!("resource {} not found!", path))
            .clone()
    }
    pub fn load_image(&self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();

        let img = map2anyhow_error!(
            self.load_bytes_from_memory(&path.to_string_lossy().to_string())
                .to_image(),
            "load_image failed"
        )?;
        self.image
            .load(path.to_string_lossy().to_string(), Arc::new(img));

        Ok(())
    }
    pub fn get_image(&self, path: impl Into<PathBuf>) -> anyhow::Result<Arc<DynamicImage>> {
        let path: PathBuf = path.into();
        if !self.image.exists(&path.to_string_lossy().to_string()) {
            self.load_image(&path)?;
        }
        let id = path.to_string_lossy().to_string();
        self.image.get(&id)
    }
    pub fn load_ui_font(&self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let id = path.to_string_lossy().to_string();
        let font_data = self.load_bytes_from_memory(&path.to_string_lossy().to_string());
        let font_data = Arc::new(FontData::from_owned(font_data.as_ref().to_vec()));
        let mut egui_font = self.egui_font.write();
        egui_font.font_data.insert(id.clone(), font_data);
        egui_font
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, id.clone());
        egui_font
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
        match create_cursor(event_loop, &img) {
            Ok(c) => {
                self.window_cursor.load(name.clone(), c);
                log::debug!("cursor {} loaded!", name);
            }
            Err(err) => {
                log::error!("load cursor {} failed: {}", name, err);
            }
        }

        Ok(())
    }
    pub fn get_cursor(&self, path: &String) -> anyhow::Result<Arc<CustomCursor>> {
        self.window_cursor.get(path)
    }
    pub fn load_window_icon(&self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
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
            .load(path.to_string_lossy().to_string(), icon);
        Ok(())
    }
    pub fn get_window_icon(&self, path: &String) -> anyhow::Result<Arc<Icon>> {
        if !self.window_icon.exists(path) {
            self.load_window_icon(path)?;
        }
        self.window_icon.get(path)
    }
    pub fn load_ui_texture(
        &self,
        path: impl Into<PathBuf>,
        ctx: &egui::Context,
    ) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let id = path.to_string_lossy().to_string();
        let img = self.get_image(&path)?;
        let texture = texture_from_image(&id, &img, ctx)?;
        self.egui_texture.load(id, texture);
        Ok(())
    }
    pub fn get_ui_texture(&self, path: &String) -> anyhow::Result<TextureHandle> {
        self.egui_texture.get(path)
    }
}
