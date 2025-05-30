use crate::map2anyhow_error;
use image::DynamicImage;
use std::{collections::HashMap, path::PathBuf};
pub mod lua;
pub mod types;
#[cfg(not(feature = "debug"))]
use packtool::MemResource;
pub enum Resource {
    Image(DynamicImage),
}
pub struct ResourceManager {
    resources: HashMap<String, Resource>,
    #[cfg(not(feature = "debug"))]
    pub memory_resource: MemResource,
    #[cfg(feature = "debug")]
    assets_path: PathBuf,
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
}
