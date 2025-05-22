use path_slash::PathExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
use zstd::stream::{decode_all, Encoder};

const MAGIC: &[u8; 4] = b"GPAC";
const VERSION: u16 = 1;
type Sha256Digest = [u8; 32];
#[derive(Serialize, Deserialize, Debug, Default)]
struct PackageHeader {
    magic: [u8; 4],
    version: u16,
    file_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileEntry {
    path: String,
    data_offset: u64,
    data_length: u64,
    compressed: bool,
    hash: Sha256Digest,
}

#[derive(Default, Debug)]
pub struct GamePackage {
    files: HashMap<String, Vec<u8>>,
    header: PackageHeader,
}
impl Display for GamePackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "{:?}", self.header);
        for (name, bytes) in self.files.iter() {
            let _ = writeln!(f, "name: {}, size: {}", name, bytes.len());
        }
        Ok(())
    }
}
impl GamePackage {
     fn add_file(&mut self, path: &str, data: &[u8]) {
        self.files.insert(path.to_string(), data.to_vec());
    }
    pub fn add_folder(&mut self, root: impl Into<PathBuf>) -> anyhow::Result<()> {
        let base: PathBuf = root.into();
        for entry in walkdir::WalkDir::new(&base)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let rel_path = path
                .strip_prefix(&base)?
                .to_slash_lossy();
            let data = std::fs::read(path)?;
            self.add_file(&rel_path, &data);
        }
        Ok(())
    }
    pub fn write_to_file(&self, path: impl Into<PathBuf>, compress: bool) -> anyhow::Result<()> {
        let mut out = File::create(path.into())?;
        let mut offset = 0u64;
        let mut entries = Vec::new();
        let mut data_block = Vec::new();

        for (path, content) in &self.files {
            let mut data = content.clone();
            if compress {
                let mut encoder = Encoder::new(Vec::new(), 0)?;
                encoder.write_all(&data)?;
                data = encoder.finish()?;
            }

            let hash = Sha256::digest(&content);
            entries.push(FileEntry {
                path: path.clone(),
                data_offset: offset,
                data_length: data.len() as u64,
                compressed: compress,
                hash: hash.into(),
            });
            offset += data.len() as u64;
            data_block.extend_from_slice(&data);
        }

        let header = PackageHeader {
            magic: *MAGIC,
            version: VERSION,
            file_count: entries.len() as u32,
        };

        let entry_bytes = bincode::serialize(&entries)?;
        let header_bytes = bincode::serialize(&header)?;

        out.write_all(&(header_bytes.len() as u64).to_le_bytes())?;
        out.write_all(&header_bytes)?;
        out.write_all(&(entry_bytes.len() as u64).to_le_bytes())?;
        out.write_all(&entry_bytes)?;
        out.write_all(&data_block)?;
        Ok(())
    }
    pub fn read_from_file(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let mut file = File::open(path.into())?;
        let mut len_buf = [0u8; 8];

        file.read_exact(&mut len_buf)?;
        let header_len = u64::from_le_bytes(len_buf);
        let mut header_buf = vec![0; header_len as usize];
        file.read_exact(&mut header_buf)?;
        let header: PackageHeader = bincode::deserialize(&header_buf)?;
        if header.magic != *MAGIC {
            return Err(anyhow::anyhow!("Wrong Pak format, mismatch MAGIC!"));
        }
        file.read_exact(&mut len_buf)?;
        let entries_len = u64::from_le_bytes(len_buf);
        let mut entry_buf = vec![0; entries_len as usize];
        file.read_exact(&mut entry_buf)?;
        let entries: Vec<FileEntry> = bincode::deserialize(&entry_buf)?;
        let mut files = HashMap::new();
        for entry in entries {
            file.seek(SeekFrom::Start(
                8 + header_len + 8 + entries_len + entry.data_offset,
            ))?;
            let mut data = vec![0; entry.data_length as usize];
            file.read_exact(&mut data)?;

            if entry.compressed {
                data = decode_all(&*data)?;
            }
            let hash = Sha256::digest(&data);
            if hash == entry.hash.into() {
                files.insert(entry.path.clone(), data);
                log::debug!("add resource {} !", entry.path)
            }
            else {
                log::error!("Sha256 check for {} failed!", entry.path)
            }
        }

        Ok(Self { files, header })
    }

    pub fn extract(&self, path: impl Into<String>) -> Option<&[u8]> {
        self.files.get(&path.into()).map(|v| &v[..])
    }
}

