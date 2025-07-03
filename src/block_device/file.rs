use crate::block_device::BlockDevice;
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

#[derive(Debug)]
pub struct FileBlockDevice {
    file: File,
}

impl FileBlockDevice {
    pub fn open_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        File::open(path).map(Self::from_file)
    }

    pub fn from_file(file: File) -> Self {
        Self { file }
    }

    pub fn try_clone(&self) -> Result<Self> {
        Ok(Self {
            file: self.file.try_clone()?,
        })
    }
}

impl BlockDevice for FileBlockDevice {
    fn size(&self) -> Result<u64> {
        self.file.metadata().map(|metadata| metadata.size())
    }
}

impl Read for FileBlockDevice {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for FileBlockDevice {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.file.seek(pos)
    }
}
