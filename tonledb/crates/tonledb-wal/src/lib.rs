use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};


pub struct Wal { file: File }
impl Wal {
pub fn open(path: &str) -> anyhow::Result<Self> {
let file = OpenOptions::new().create(true).read(true).append(true).open(path)?; Ok(Self { file })
}
pub fn append(&mut self, bytes: &[u8]) -> anyhow::Result<()> { self.file.write_all(bytes)?; self.file.write_all(b"\n")?; self.file.flush()?; Ok(()) }
pub fn replay(&mut self) -> anyhow::Result<Vec<Vec<u8>>> {
let mut buf = Vec::new(); self.file.seek(SeekFrom::Start(0))?; self.file.read_to_end(&mut buf)?;
Ok(buf.split(|b| *b==b'\n').filter(|r|!r.is_empty()).map(|r| r.to_vec()).collect())
}
}