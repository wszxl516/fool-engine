use crate::map2anyhow_error;
use nannou::image::{self, DynamicImage};
use nannou::wgpu::DeviceQueuePair;
use nannou_egui::egui::Context;
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
    #[cfg(not(feature = "debug"))]
    pub memory_resource: packtool::ResourcePackage,
    #[cfg(feature = "debug")]
    assets_path: PathBuf,
    dev: Arc<DeviceQueuePair>,
    ctx: Context,
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
    pub fn new(dev: &Arc<DeviceQueuePair>, ctx: Context) -> anyhow::Result<Self> {
        let assets_path = resource_path()?;
        #[cfg(not(feature = "debug"))]
        let resource_pack =
            packtool::ResourcePackage::unpack_from_file(assets_path.join("assets.pak"))?;
        Ok(Self {
            resources: HashMap::new(),
            #[cfg(not(feature = "debug"))]
            memory_resource: resource_pack,
            dev: dev.clone(),
            #[cfg(feature = "debug")]
            assets_path,
            ctx,
        })
    }
    #[cfg(not(feature = "debug"))]
    pub fn load_bytes_from_memory(&self, path: &String) -> &[u8] {
        self.memory_resource
            .get_file(path)
            .expect(&format!("resource {} not found!", path))
    }
    #[cfg(not(feature = "debug"))]
    pub fn all_memory_resource(&self) -> &HashMap<String, Vec<u8>> {
        self.memory_resource.all_resource()
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
                _ => None,
            })
            .ok_or_else(|| anyhow::anyhow!("resource is not a image or not found!"))
    }
    pub fn load_font(&mut self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let path: PathBuf = path.into();
        #[cfg(feature = "debug")]
        {
            use std::io::Read;
            let mut fd = map2anyhow_error!(
                std::fs::File::open(self.assets_path.join(&path)),
                "load_font failed"
            )?;
            let mut buffer = Vec::new();
            fd.read_to_end(&mut buffer)?;
            let font = LuaFont::from_bytes(buffer, &path.to_string_lossy().to_string())?;
            self.resources
                .insert(path.to_string_lossy().to_string(), Resource::Font(font));
        }
        #[cfg(not(feature = "debug"))]
        {
            let buffer = self.load_bytes_from_memory(&path.to_string_lossy().to_string());
            let font = LuaFont::from_bytes(buffer.into(), &path.to_string_lossy().to_string())?;
            self.resources
                .insert(path.to_string_lossy().to_string(), Resource::Font(font));
        }

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
