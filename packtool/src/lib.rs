use chrono::{DateTime, Utc};
use path_slash::PathExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};
use zstd::stream::{Encoder, decode_all};

const MAGIC: &[u8; 4] = b"GPAC";
const VERSION: [u8; 4] = [0, 0, 0, 1];
type Sha256Digest = [u8; 32];
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PackageHeader {
    pub magic: [u8; 4],
    pub version: [u8; 4],
    pub file_count: u32,
    pub compress: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}
impl PackageHeader {
    pub fn version_string(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.version[0], self.version[1], self.version[2], self.version[3]
        )
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FileEntry {
    pub path: String,
    pub data_offset: u64,
    pub data_length: u64,
    pub hash: Sha256Digest,
}

#[derive(Default, Debug)]
pub struct ResourcePackage {
    files: HashMap<String, Vec<u8>>,
    pub entrys: Vec<FileEntry>,
    header: PackageHeader,
    pub input: PathBuf,
    pub output: PathBuf,
    pub compress: bool,
    pub total_size: u64,
}
impl Display for ResourcePackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "{:?}", self.header);
        for (name, bytes) in self.files.iter() {
            let _ = writeln!(f, "name: {}, size: {}", name, bytes.len());
        }
        Ok(())
    }
}
impl ResourcePackage {
    pub fn new(input: impl Into<PathBuf>, output: impl Into<PathBuf>, compress: bool) -> Self {
        Self {
            files: Default::default(),
            header: Default::default(),
            input: input.into(),
            output: output.into(),
            compress,
            total_size: 0,
            entrys: Default::default(),
        }
    }
    fn add_file(&mut self, path: &str, data: &[u8]) {
        self.files.insert(path.to_string(), data.to_vec());
    }
    fn add_folder(&mut self) -> anyhow::Result<()> {
        let base: PathBuf = self.input.clone();
        for entry in walkdir::WalkDir::new(&base)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let rel_path = path.strip_prefix(&base)?.to_slash_lossy();
            let data = std::fs::read(path)?;
            self.add_file(&rel_path, &data);
        }
        Ok(())
    }
    pub fn pack(&mut self) -> anyhow::Result<()> {
        self.add_folder()?;
        let path = self.output.clone();
        if let Some(p) = path.parent() {
            println!("{}", p.display());
            p.exists().eq(&false).then(|| std::fs::create_dir_all(&p));
        }
        let mut out = File::create(path)?;
        let mut offset = 0u64;
        let mut entries = Vec::new();
        let mut data_block = Vec::new();

        for (path, content) in &self.files {
            let mut data = content.clone();
            if self.compress {
                let mut encoder = Encoder::new(Vec::new(), 0)?;
                encoder.write_all(&data)?;
                data = encoder.finish()?;
            }

            let hash = Sha256::digest(&content);
            entries.push(FileEntry {
                path: path.clone(),
                data_offset: offset,
                data_length: data.len() as u64,
                hash: hash.into(),
            });
            offset += data.len() as u64;
            data_block.extend_from_slice(&data);
        }

        self.header = PackageHeader {
            magic: *MAGIC,
            version: VERSION,
            file_count: entries.len() as u32,
            compress: self.compress,
            timestamp: Utc::now(),
        };
        let entry_bytes = bincode::serialize(&entries)?;
        let header_bytes = bincode::serialize(&self.header)?;

        out.write_all(&(header_bytes.len() as u64).to_le_bytes())?;
        out.write_all(&header_bytes)?;
        out.write_all(&(entry_bytes.len() as u64).to_le_bytes())?;
        out.write_all(&entry_bytes)?;
        out.write_all(&data_block)?;
        out.sync_data()?;
        self.total_size = out.metadata().and_then(|x| Ok(x.size())).unwrap_or(0);
        Ok(())
    }
    pub fn unpack_from_file(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();
        let mut file = File::open(&path)?;
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
        for entry in &entries {
            file.seek(SeekFrom::Start(
                8 + header_len + 8 + entries_len + entry.data_offset,
            ))?;
            let mut data = vec![0; entry.data_length as usize];
            file.read_exact(&mut data)?;

            if header.compress {
                data = decode_all(&*data)?;
            }
            let hash = Sha256::digest(&data);
            if hash == entry.hash.into() {
                files.insert(entry.path.clone(), data);
                log::debug!("add resource {} !", entry.path)
            } else {
                log::error!("Sha256 check for {} failed!", entry.path)
            }
        }

        Ok(Self {
            compress: header.compress,
            input: path.into(),
            files,
            header,
            output: Default::default(),
            total_size: file.metadata().and_then(|x| Ok(x.size())).unwrap_or(0),
            entrys: entries,
        })
    }
    pub fn get_info_from_file(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();
        let mut file = File::open(&path)?;
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
        Ok(Self {
            compress: header.compress,
            input: path.into(),
            files: Default::default(),
            header,
            output: Default::default(),
            total_size: file.metadata().and_then(|x| Ok(x.size())).unwrap_or(0),
            entrys: entries,
        })
    }
    pub fn unpack2dir(&self, output: impl Into<PathBuf>) -> anyhow::Result<()> {
        let out: PathBuf = output.into();
        out.exists()
            .eq(&false)
            .then(|| std::fs::create_dir_all(&out));
        for (name, data) in &self.files {
            let full_path = out.clone().join(name);
            match full_path.parent() {
                Some(p) => {
                    log::debug!("create dir :{}", p.display());
                    p.exists().eq(&false).then(|| std::fs::create_dir_all(&p));
                }
                None => {}
            }
            let mut f = std::fs::File::options()
                .write(true)
                .create_new(true)
                .write(true)
                .open(&full_path)?;
            f.write_all(&data)?;
            log::debug!("unpack file: {}, size: {}", full_path.display(), data.len());
        }
        Ok(())
    }
    pub fn get_file(&self, path: impl Into<String>) -> Option<&[u8]> {
        self.files.get(&path.into()).map(|v| &v[..])
    }
    pub fn all_resource(&self) -> &HashMap<String, Vec<u8>> {
        &self.files
    }
    pub fn info(&self) -> &PackageHeader {
        &self.header
    }
}
