pub use crate::utils::dir::{current_exe_path, current_run_path, load_from_current};
mod base;
mod window;
use serde::{Deserialize, Serialize};

pub use base::BaseConfig;
pub use window::WindowConfig;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub base: BaseConfig,
    pub window: WindowConfig,
}

impl Config {
    pub fn from_file() -> anyhow::Result<Self> {
        let current_path = current_exe_path()?.join("config.toml");
        let buffer = load_from_current("config.toml")?;
        let config: Self = toml::from_str(&String::from_utf8(buffer)?).map_err(|err| {
            anyhow::anyhow!("failed parse config {}: {}", current_path.display(), err)
        })?;
        Ok(config)
    }
}
