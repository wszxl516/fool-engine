use nannou::image::{self, DynamicImage};
use nannou::wgpu::DeviceQueuePair;
use nannou_egui::egui::Context;
use std::io::Read;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
pub mod lua;
pub mod types;
use types::{LuaFont, LuaTexture};
pub enum Resource {
    Image(DynamicImage),
    Font(LuaFont),
    UITexture(LuaTexture),
}
pub struct ResourceManager {
    resources: HashMap<String, Resource>,
    assets_path: PathBuf,
    dev: Arc<DeviceQueuePair>,
    ctx: Context,
}

impl ResourceManager {
    pub fn new(assets_path: &PathBuf, dev: &Arc<DeviceQueuePair>, ctx: Context) -> Self {
        Self {
            resources: HashMap::new(),
            assets_path: assets_path.clone(),
            dev: dev.clone(),
            ctx,
        }
    }
    pub fn load_image(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let img = image::open(self.assets_path.join(&path))
            .map_err(|e| anyhow::anyhow!(format!("load_image failed: {}", e)))?;
        self.resources
            .insert(path.to_string_lossy().to_string(), Resource::Image(img));
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
                _ => None,
            })
            .ok_or_else(|| anyhow::anyhow!("resource is not a image or not found!"))
    }
    pub fn load_font(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let mut fd = std::fs::File::open(self.assets_path.join(&path))
            .map_err(|e| anyhow::anyhow!(format!("load_font failed: {}", e)))?;
        let mut buffer = Vec::new();
        fd.read_to_end(&mut buffer)?;
        let font = LuaFont::from_bytes(buffer, &path.to_string_lossy().to_string())?;
        self.resources
            .insert(path.to_string_lossy().to_string(), Resource::Font(font));
        Ok(())
    }
    pub fn get_font(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<LuaFont> {
        let path: PathBuf = path.into();
        let id = path.to_string_lossy().to_string();
        if !self.resources.contains_key(&id) {
            self.load_font(&path)?;
        }
        match self.resources.get(&id) {
            Some(resource) => {
                if let Resource::Font(font) = resource {
                    Ok(font.clone())
                } else {
                    Err(anyhow::anyhow!("resource {} not a font!", path.display()))
                }
            }
            None => Err(anyhow::anyhow!("resource {} not found!", path.display())),
        }
    }
    fn load_texture(
        &mut self,
        path: impl Into<PathBuf>,
        img_path: impl Into<PathBuf>,
    ) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        let img_path: PathBuf = img_path.into();
        let dev = self.dev.clone();
        let ctx = self.ctx.clone();
        let img = self.get_image(img_path)?;
        let texture = LuaTexture::from_image(&path.to_string_lossy().to_string(), img, &dev, ctx)?;
        self.resources.insert(
            path.to_string_lossy().to_string(),
            Resource::UITexture(texture),
        );
        Ok(())
    }
    pub fn get_texture(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<LuaTexture> {
        let path: PathBuf = path.into();
        let id = path.to_string_lossy().to_string() + "_texture";
        if !self.resources.contains_key(&id) {
            self.load_texture(&id, &path)?;
        }
        match self.resources.get(&id) {
            Some(resource) => {
                if let Resource::UITexture(texture) = resource {
                    Ok(texture.clone())
                } else {
                    Err(anyhow::anyhow!(
                        "resource {} not a texture!",
                        path.display()
                    ))
                }
            }
            None => Err(anyhow::anyhow!("resource {} not found!", path.display())),
        }
    }
}
