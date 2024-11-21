use crate::block_device::BlockDevice;
use crate::ioctl::blk::BLK_GETSIZE64;
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct NativeBlockDevice {
    file: File,
}

impl NativeBlockDevice {
    pub fn open_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        File::open(path).map(Self::from_file)
    }

    pub fn from_file(file: File) -> Self {
        Self { file }
    }
}

impl BlockDevice for NativeBlockDevice {
    fn size(&self) -> Result<u64> {
        BLK_GETSIZE64.ioctl(&self.file).map(|(_, size)| size)
    }
}

impl Read for NativeBlockDevice {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for NativeBlockDevice {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.file.seek(pos)
    }
}
