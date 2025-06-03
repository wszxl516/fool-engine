use fool_resource::{Fallback, SharedData};
use std::io::Read;
#[derive(Debug, Clone)]
pub struct FSFallBack {
    pub asset_path: std::path::PathBuf,
}
impl Fallback for FSFallBack {
    type K = String;
    type V = SharedData;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let full_path = self.asset_path.join(key);
        let mut fd = std::fs::File::open(full_path)?;
        let mut buffer = Vec::new();
        fd.read_to_end(&mut buffer)?;
        Ok(SharedData::from(buffer))
    }
}
