use crate::create_if_not_exists;
use crate::utils::dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseConfig {
    pub name: String,
    pub capture_path: PathBuf,
    pub save_path: PathBuf,
    pub assets_path: PathBuf,
    pub fps: u32,
}
impl BaseConfig {
    pub fn build(&self) -> anyhow::Result<Self> {
        let usr_dir = directories::UserDirs::new();
        let current_dir = dir::current_exe_path()?;
        if let Some(d) = usr_dir {
            let pic_dir = d
                .picture_dir()
                .unwrap_or(current_dir.as_path())
                .join(&self.name)
                .join(self.capture_path.clone())
                .to_path_buf();
            let save_dir = d
                .document_dir()
                .unwrap_or(current_dir.as_path())
                .join(&self.name)
                .join(self.save_path.clone())
                .to_path_buf();
            let assets_dir = current_dir
                .to_path_buf()
                .join(self.assets_path.to_path_buf());
            create_if_not_exists!(&pic_dir)?;
            create_if_not_exists!(&save_dir)?;
            Ok(Self {
                name: self.name.clone(),
                capture_path: pic_dir,
                save_path: save_dir,
                assets_path: assets_dir,
                fps: self.fps,
            })
        } else {
            Err(anyhow::anyhow!("failed to get base path for Fool Engine!"))
        }
    }
}
