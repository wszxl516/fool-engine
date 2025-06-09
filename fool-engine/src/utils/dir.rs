use std::{io::Read, path::PathBuf};
pub fn current_exe_path() -> anyhow::Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    exe_path
        .parent()
        .ok_or(anyhow::anyhow!("failed get current path!"))
        .and_then(|p| Ok(p.to_path_buf()))
}

pub fn current_run_path() -> anyhow::Result<PathBuf> {
    Ok(std::env::current_dir()?)
}

pub fn load_from_current(name: &str) -> anyhow::Result<Vec<u8>> {
    let paths = [
        current_exe_path()?.join(name),
        current_run_path()?.join(name),
    ];

    for path in &paths {
        if path.exists() {
            let mut buffer = Vec::new();
            std::fs::File::open(path)?.read_to_end(&mut buffer)?;
            return Ok(buffer);
        }
    }

    Err(anyhow::anyhow!(
        "File '{}' not found in exe or current dir",
        name
    ))
}
#[macro_export]
macro_rules! create_if_not_exists {
    ($path:expr) => {{
        if !$path.exists() {
            std::fs::create_dir_all(&$path)
        } else {
            Ok(())
        }
    }};
}
