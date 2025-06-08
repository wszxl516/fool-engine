pub mod utils;
mod window;

use serde::{Deserialize, Serialize};

pub use window::WindowConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
}

impl Config {
    pub fn from_file() -> anyhow::Result<Self> {
        let current_path = utils::current_exe_path()?.join("config.toml");
        let buffer = utils::load_from_current("config.toml")?;
        let config: Self = toml::from_str(&String::from_utf8(buffer)?).map_err(|err| {
            anyhow::anyhow!("failed parse config {}: {}", current_path.display(), err)
        })?;
        Ok(config)
    }
}
