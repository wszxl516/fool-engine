mod tee;
use chrono::{DateTime, Utc};
use path_slash::PathExt;
use serde::{Deserialize, Serialize};
#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufWriter, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
use tee::{TeeReader, TeeWriter, WriteCounter};
use zstd::stream::{Decoder, Encoder};
const MAGIC: &[u8; 4] = b"GPAC";
const VERSION: [u8; 4] = [0, 0, 0, 1];
const FOOTER_MAGIC: &[u8; 7] = b"GPACEND";
const FOOTER_LEN: usize = size_of::<u64>() * 2 + FOOTER_MAGIC.len();
type Sha256Digest = [u8; 32];
pub type MemResource = HashMap<String, Vec<u8>>;
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PackageHeader {
    pub magic: [u8; 4],
    pub version: [u8; 4],
    pub file_count: u32,
    pub compress: bool,
    pub compress_level: i32,
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
    pub header: PackageHeader,
    files: HashMap<String, PathBuf>,
    pub entrys: Vec<FileEntry>,
    pub input: PathBuf,
    pub output: PathBuf,
    pub total_size: u64,
}

impl Display for ResourcePackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "{:?}", self.header);
        for (name, path) in self.files.iter() {
            let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            let _ = writeln!(f, "name: {}, size: {}", name, size);
        }
        Ok(())
    }
}

impl ResourcePackage {
    pub fn create_pak(
        input: impl Into<PathBuf>,
        output: impl Into<PathBuf>,
        compress: bool,
        compress_level: i32,
    ) -> Self {
        Self {
            files: Default::default(),
            header: PackageHeader {
                magic: *MAGIC,
                version: VERSION,
                file_count: 0,
                compress: compress,
                compress_level: compress_level,
                timestamp: Default::default(),
            },
            input: input.into(),
            output: output.into(),
            total_size: 0,
            entrys: Default::default(),
        }
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
            self.files.insert(rel_path.to_string(), path.to_path_buf());
        }
        Ok(())
    }

    pub fn pack(&mut self) -> anyhow::Result<()> {
        self.add_folder()?;
        let path = self.output.clone();
        if let Some(p) = path.parent() {
            if !p.exists() {
                std::fs::create_dir_all(&p)?;
            }
        }
        let mut out_file = File::create(&path)?;
        let mut entries = Vec::new();
        {
            let mut offset = 0u64;

            for (rel_path, real_path) in &self.files {
                let mut out = WriteCounter::new(&mut out_file);
                let file = File::open(real_path)?;
                let mut tee_reader = TeeReader::new(file);
                if self.header.compress {
                    let mut encoder = Encoder::new(&mut out, self.header.compress_level)?;
                    std::io::copy(&mut tee_reader, &mut encoder)?;
                    encoder.finish()?;
                } else {
                    std::io::copy(&mut tee_reader, &mut out)?;
                };
                let entry = FileEntry {
                    path: rel_path.clone(),
                    data_offset: offset,
                    data_length: out.bytes_written(),
                    hash: tee_reader.finalize(),
                };
                log::debug!(
                    "add {} to pack size: {}, offset: {}",
                    rel_path,
                    out.bytes_written(),
                    offset
                );
                offset += out.bytes_written();
                entries.push(entry);
            }
        }
        self.header.file_count = entries.len() as u32;
        self.header.timestamp = Utc::now();

        let entry_bytes = bincode::serialize(&entries)?;
        let header_bytes = bincode::serialize(&self.header)?;

        out_file.write_all(&header_bytes)?;
        out_file.write_all(&entry_bytes)?;

        let footer = {
            let header_len = header_bytes.len() as u64;
            let entry_len = entry_bytes.len() as u64;
            let mut f = Vec::new();
            f.extend(&header_len.to_le_bytes());
            f.extend(&entry_len.to_le_bytes());
            f.extend(FOOTER_MAGIC);
            f
        };
        out_file.write_all(&footer)?;

        out_file.sync_data()?;
        #[cfg(target_os = "linux")]
        {
            self.total_size = out_file.metadata()?.size();
        }
        Ok(())
    }
}
impl ResourcePackage {
    pub fn from_pak(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();
        let mut file = File::open(&path)?;
        let mut footer = [0u8; FOOTER_LEN];
        file.seek(SeekFrom::End(-(FOOTER_LEN as i64)))?;
        file.read_exact(&mut footer)?;

        if &footer[16..] != FOOTER_MAGIC {
            return Err(anyhow::anyhow!("Invalid package: missing footer magic"));
        }

        let header_len = u64::from_le_bytes(footer[0..8].try_into().unwrap());
        let entry_len = u64::from_le_bytes(footer[8..16].try_into().unwrap());

        let mut buf = vec![0u8; (header_len + entry_len) as usize];
        file.seek(SeekFrom::End(-(FOOTER_LEN as i64 + buf.len() as i64)))?;
        file.read_exact(&mut buf)?;

        let header: PackageHeader = bincode::deserialize(&buf[..header_len as usize])?;
        let entries: Vec<FileEntry> = bincode::deserialize(&buf[header_len as usize..])?;
        #[cfg(target_os = "linux")]
        let size = file.metadata().and_then(|s| Ok(s.size())).unwrap_or(0);
        #[cfg(not(target_os = "linux"))]
        let size = 0;
        Ok(Self {
            files: HashMap::new(),
            entrys: entries,
            header,
            input: path,
            output: PathBuf::new(),
            total_size: size,
        })
    }
    pub fn unpack2dir(&self, output: impl Into<PathBuf>) -> anyhow::Result<()> {
        let out: PathBuf = output.into();
        if !out.exists() {
            std::fs::create_dir_all(&out)?;
        }

        let mut file = File::open(&self.input)?;
        for entry in &self.entrys {
            let full_path = out.join(&entry.path);
            if let Some(p) = full_path.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            file.seek(SeekFrom::Start(entry.data_offset))?;
            let mut f = std::fs::File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&full_path)?;
            let mut writer = TeeWriter::new(&mut f);
            let mut sized_file = std::io::Read::by_ref(&mut file).take(entry.data_length);
            if self.header.compress {
                let mut decoder = Decoder::new(&mut sized_file)?;
                std::io::copy(&mut decoder, &mut writer)?;
            } else {
                std::io::copy(&mut sized_file, &mut writer)?;
            }
            writer.flush()?;
            if !(entry.hash == writer.finalize()) {
                anyhow::bail!("SHA256 checksum mismatch for file: {}", full_path.display());
            }
            f.flush()?;
            #[cfg(target_os = "linux")]
            let size = f.metadata().and_then(|s| Ok(s.size())).unwrap_or(0);
            #[cfg(not(target_os = "linux"))]
            let size = 0;
            log::debug!("unpack file: {}, size: {}", full_path.display(), size);
        }

        Ok(())
    }
    pub fn unpack2memory(&self) -> anyhow::Result<HashMap<String, Vec<u8>>> {
        let mut file = File::open(&self.input)?;
        let mut resource = HashMap::default();
        for entry in &self.entrys {
            file.seek(SeekFrom::Start(entry.data_offset))?;
            let mut mem = BufWriter::new(Vec::new());
            let hash = {
                let mut writer = TeeWriter::new(&mut mem);
                let mut sized_file = std::io::Read::by_ref(&mut file).take(entry.data_length);
                if self.header.compress {
                    let mut decoder = Decoder::new(&mut sized_file)?;
                    std::io::copy(&mut decoder, &mut writer)?;
                } else {
                    std::io::copy(&mut sized_file, &mut writer)?;
                }
                writer.flush()?;
                writer.finalize()
            };
            let buffer = mem.into_inner()?;
            if !(entry.hash == hash) {
                anyhow::bail!("SHA256 checksum mismatch for file: {}", entry.path);
            }
            log::debug!("unpack file: {}, size: {}", entry.path, buffer.len());
            resource.insert(entry.path.clone(), buffer);
        }

        Ok(resource)
    }
    pub fn info(&self) -> &PackageHeader {
        &self.header
    }
}
