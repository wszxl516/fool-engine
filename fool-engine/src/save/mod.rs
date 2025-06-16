use bincode::{Decode, Encode};
use bson::Bson;
use chrono::{DateTime, Local, Utc};
use fool_script::modules::ser::{bson_to_lua_value, lua_value_to_bson};
use mlua::{UserData, Value};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    io::{Read, Write},
    path::PathBuf,
};

use crate::{lua_create_table, map2lua_error};
const MAGIC: [u8; 4] = [b'f', b'o', b'o', b'l'];
const VERSION: [u8; 4] = [0, 0, 0, 1];

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Header {
    magic: [u8; 4],
    version: [u8; 4],
    compress: bool,
}

impl Header {
    pub fn is_vaild(&self) -> bool {
        self.magic == MAGIC && self.version == VERSION
    }
    pub fn read<R: Read>(f: &mut R) -> anyhow::Result<Self> {
        Ok(bincode::decode_from_std_read(
            f,
            bincode::config::standard(),
        )?)
    }
    pub fn write<W: Write>(&self, f: &mut W) -> anyhow::Result<()> {
        bincode::encode_into_std_write(self, f, bincode::config::standard())?;
        Ok(())
    }
}

impl Header {
    fn with_compress(compress: bool) -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
            compress,
        }
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Entry {
    pub name: Option<String>,
    pub create_at: DateTime<Utc>,
    pub data: Bson,
}
impl Entry {
    pub fn load<R: Read>(r: &mut R, compress: bool) -> anyhow::Result<Self> {
        if compress {
            let decoder = zstd::Decoder::new(r)?;
            Ok(bson::from_reader(decoder)?)
        } else {
            Ok(bson::from_reader(r)?)
        }
    }
    pub fn save(
        path: impl Into<PathBuf>,
        name: Option<impl Into<String> + Clone>,
        data: Bson,
        compress: bool,
    ) -> anyhow::Result<()> {
        let date = Utc::now().with_timezone(&Utc);
        let entry = Entry {
            name: name.clone().map(|x| x.into()),
            create_at: date,
            data,
        };
        let name = if let Some(name) = name {
            name.into()
        } else {
            date.with_timezone(&Local)
                .format("%Y-%m-%d-%H-%M-%S%.3f")
                .to_string()
        };
        let full_path = path.into().join(format!("{}.save", name));
        let mut fd = std::fs::File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(full_path)?;
        Header::with_compress(compress).write(&mut fd)?;
        let data = bson::to_vec(&entry)?;
        if compress {
            let mut encoder = zstd::Encoder::new(fd, 10)?;
            encoder.write_all(&data)?;
            encoder.finish()?;
        } else {
            fd.write_all(&data)?;
        }
        Ok(())
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.create_at == other.create_at
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.create_at.cmp(&other.create_at)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaveManager {
    path: PathBuf,
}

impl SaveManager {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        log::debug!("SaveManager init from {}", path.display());
        Self { path: path }
    }

    pub fn list(&self) -> anyhow::Result<Vec<Entry>> {
        let mut entrys = Vec::new();
        for entry_res in std::fs::read_dir(&self.path)? {
            if let Ok(entry) = entry_res {
                if !entry.path().extension().map_or(false, |ext| ext == "save") {
                    continue;
                }
                let mut fd = std::fs::File::open(entry.path())?;
                let header = Header::read(&mut fd)?;
                if header.is_vaild() {
                    match Entry::load(&mut fd, header.compress) {
                        Ok(e) => {
                            log::debug!("save file {} loaded!", entry.path().display());
                            entrys.push(e)
                        }
                        Err(err) => {
                            log::debug!("save file {} load failed: {}", entry.path().display(), err)
                        }
                    }
                }
            }
        }
        Ok(entrys)
    }
    pub fn load(&self, name: impl Into<String> + Clone) -> anyhow::Result<Entry> {
        let name = name.into();
        let full_path = self.path.join(format!("{}.save", &name));
        let mut fd = std::fs::File::open(full_path)?;
        let header = Header::read(&mut fd)?;
        if header.is_vaild() {
            Ok(Entry::load(&mut fd, header.compress)?)
        } else {
            Err(anyhow::anyhow!("{} not found!", name))
        }
    }
    pub fn save(
        &self,
        name: Option<impl Into<String> + Clone>,
        compress: bool,
        data: Bson,
    ) -> anyhow::Result<()> {
        Entry::save(&self.path, name, data, compress)
    }
    pub fn delete(&self, name: &str) -> anyhow::Result<()> {
        let path = self.path.join(format!("{}.save", name));
        std::fs::remove_file(path)?;
        Ok(())
    }
}

impl UserData for SaveManager {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "save",
            |_lua, this, (name, data): (Option<String>, Value)| {
                let data = lua_value_to_bson(data)?;
                map2lua_error!(this.save(name, true, data), "SaveManager::save")?;
                Ok(())
            },
        );
        methods.add_method("delete", |_lua, this, name: String| {
            map2lua_error!(this.delete(&name), "SaveManager::delete")?;
            Ok(())
        });
        methods.add_method("list", |lua, this, ()| {
            let entrys = map2lua_error!(this.list(), "SaveManager::list")?;
            let lua_entrys = lua_create_table!(lua, []);
            for (index, entry) in entrys.iter().enumerate() {
                let local_str = entry
                    .create_at
                    .with_timezone(&Local)
                    .format("%Y-%m-%d-%H-%M-%S%.3f")
                    .to_string();
                let name = entry.name.clone().unwrap_or(local_str.clone());
                let data = bson_to_lua_value(lua, &entry.data)?;
                let lua_entry =
                    lua_create_table!(lua, [name = name, create_at = local_str, data = data]);
                lua_entrys.set(index + 1, lua_entry)?;
            }
            Ok(lua_entrys)
        });
        methods.add_method("load", |lua, this, name: String| {
            match map2lua_error!(this.load(&name), "SaveManager::load") {
                Ok(entry) => {
                    let local_str = entry
                        .create_at
                        .with_timezone(&Local)
                        .format("%Y-%m-%d-%H-%M-%S%.3f")
                        .to_string();
                    let name = entry.name.clone().unwrap_or(local_str.clone());
                    let data = bson_to_lua_value(lua, &entry.data)?;
                    let lua_entry =
                        lua_create_table!(lua, [name = name, create_at = local_str, data = data]);
                    Ok(Value::Table(lua_entry))
                }
                Err(err) => {
                    log::error!("SaveManager load {} failed: {}", name, err);
                    Ok(Value::Nil)
                }
            }
        });
    }
}

#[test]
fn test_save() -> anyhow::Result<()> {
    let sm = SaveManager::new("/home/sun/文档/Fool Engine/saves");
    sm.save(Some("aaa"), true, Bson::Boolean(true))?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    sm.save(Some("bbb"), true, Bson::Boolean(true))?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    sm.save(Some("ccc"), true, Bson::Boolean(true))?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    sm.save(None::<String>, true, Bson::Boolean(true))?;
    let mut saves = sm.list().unwrap();
    saves.sort();
    println!("{:?}", saves);
    Ok(())
}
