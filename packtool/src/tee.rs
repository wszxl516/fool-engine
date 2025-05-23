use sha2::{Digest, Sha256};
use std::io::{self, Read, Write};

pub struct TeeReader<R> {
    reader: R,
    hasher: Sha256,
    pub total_len: usize,
}

impl<R: Read> TeeReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            hasher: Sha256::new(),
            total_len: 0,
        }
    }

    pub fn finalize(self) -> [u8; 32] {
        let result = self.hasher.finalize();
        result.into()
    }
}

impl<R: Read> Read for TeeReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.reader.read(buf)?;
        if n > 0 {
            self.total_len += n;
            self.hasher.update(&buf[..n]);
        }
        Ok(n)
    }
}

pub struct TeeWriter<R> {
    writer: R,
    hasher: Sha256,
    pub total_len: usize,
}

impl<R: Write> TeeWriter<R> {
    pub fn new(reader: R) -> Self {
        Self {
            writer: reader,
            hasher: Sha256::new(),
            total_len: 0,
        }
    }

    pub fn finalize(self) -> [u8; 32] {
        let result = self.hasher.finalize();
        result.into()
    }
}

impl<R: Write> Write for TeeWriter<R> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.writer.write(buf)?;
        if n > 0 {
            self.total_len += n;
            self.hasher.update(&buf[..n]);
        }
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

pub struct WriteCounter<W: Write> {
    inner: W,
    count: u64,
}

impl<W: Write> WriteCounter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner, count: 0 }
    }

    pub fn bytes_written(&self) -> u64 {
        self.count
    }
}

impl<W: Write> Write for WriteCounter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.count += n as u64;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
