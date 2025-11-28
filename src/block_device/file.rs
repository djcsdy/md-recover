use crate::block_device::BlockDevice;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

#[derive(Debug)]
pub struct FileBlockDevice {
    file: File,
}

impl FileBlockDevice {
    pub fn open_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        File::open(path).map(Self::from_file)
    }

    pub fn from_file(file: File) -> Self {
        Self { file }
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            file: self.file.try_clone()?,
        })
    }
}

impl BlockDevice for FileBlockDevice {
    fn block_size(&self) -> io::Result<usize> {
        Ok(4096)
    }

    fn size(&self) -> io::Result<u64> {
        self.file.metadata().map(|metadata| metadata.size())
    }

    fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            file: self.file.try_clone()?,
        })
    }
}

impl Read for FileBlockDevice {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for FileBlockDevice {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.file.seek(pos)
    }
}
