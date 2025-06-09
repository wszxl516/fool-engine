use egui::{Context, FontData, FontDefinitions};
use fool_graphics::canvas::SceneGraph;
use image::DynamicImage;
use std::{path::PathBuf, sync::Arc};
mod fallback;
pub mod types;
pub mod utils;
use egui::epaint::TextureHandle;
pub use fool_graphics::canvas::{FontManager, VelloFontFallback};
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
    pub raw_image: Resource<String, Arc<DynamicImage>>,
    pub egui_font: Arc<RwLock<FontDefinitions>>,
    pub egui_texture: Resource<String, TextureHandle>,
    pub window_cursor: Resource<String, Arc<CustomCursor>>,
    pub window_icon: Resource<String, Arc<Icon>>,
    pub graphics_font: FontManager,
    pub scene_graph: Arc<RwLock<SceneGraph>>,
}

impl ResourceManager {
    pub fn new(assets_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        // let assets_path = utils::resource_path()?;
        let assets_path = assets_path.into();
        #[cfg(feature = "debug")]
        let raw_resource = {
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
        let raw_resource: Resource<String, SharedData> = {
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
        let raw_image = Resource::<String, Arc<DynamicImage>>::empty();
        raw_image.set_fall_back(fallback::RawImageFallBack {
            raw_data: raw_resource.clone(),
        });
        let graphics_font =
            FontManager::new(VelloFontFallback::from_resource(raw_resource.clone()));
        let egui_texture = Resource::<String, TextureHandle>::empty();
        let window_icon = Resource::<String, Arc<Icon>>::empty();
        window_icon.set_fall_back(fallback::WindowIconFallBack {
            raw_image: raw_image.clone(),
        });
        Ok(Self {
            raw_image: raw_image,
            raw_resource,
            assets_path,
            egui_font: Arc::new(RwLock::new(FontDefinitions::empty())),
            window_cursor: Default::default(),
            window_icon,
            egui_texture,
            graphics_font: graphics_font.clone(),
            scene_graph: Arc::new(RwLock::new(SceneGraph {
                font_mgr: graphics_font,
                ..Default::default()
            })),
        })
    }
    pub fn setup_egui_texture_fallback(&mut self, egui_ctx: &Context) {
        let egui_texture_fallback = fallback::EguiTextureFallBack {
            ctx: egui_ctx.clone(),
            raw_image: self.raw_image.clone(),
        };
        self.egui_texture.set_fall_back(egui_texture_fallback);
    }
    pub fn load_ui_font(&self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let id = path.to_string_lossy().to_string();
        let font_data = self.raw_resource.get(&path.to_string_lossy().to_string())?;
        let font_data = Arc::new(FontData::from_owned(font_data.as_ref().to_owned()));
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
    pub fn preload_cursor(
        &mut self,
        name: &String,
        event_loop: &ActiveEventLoop,
    ) -> anyhow::Result<()> {
        let img = self.raw_image.get(name)?;
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
    pub fn preload_ui_texture(&self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let _img = self.egui_texture.get(path.to_string_lossy())?;
        Ok(())
    }
    pub fn get_ui_texture(&self, path: &String) -> anyhow::Result<TextureHandle> {
        self.egui_texture.get(path)
    }
}
