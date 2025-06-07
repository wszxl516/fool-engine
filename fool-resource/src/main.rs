use std::{collections::HashMap, io::Read};

use fool_resource::{Fallback, Resource, SharedData};

#[derive(Debug, Clone)]
struct FileFallBack {
    pub asset_path: std::path::PathBuf,
}
impl Fallback for FileFallBack {
    type K = String;
    type V = SharedData;
    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V> {
        let full_path = self.asset_path.join(key);
        println!("{:?}", full_path);
        let mut fd = std::fs::File::open(full_path)?;
        let mut buffer = Vec::new();
        fd.read_to_end(&mut buffer)?;
        Ok(SharedData::from(buffer))
    }
}
fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let mut data = HashMap::<&str, &[u8]>::new();
    data.insert("test.txt", "test".as_bytes());
    let fbk = FileFallBack {
        asset_path: "/data/works/game_engine/".into(),
    };
    let res = Resource::<String, SharedData>::empty();
    res.set_fall_back(fbk);
    res.load_from_map(data);
    println!("{:?}", res.get("script.sh"));
    Ok(())
}
