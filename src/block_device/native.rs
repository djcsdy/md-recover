use crate::block_device::BlockDevice;
use crate::ioctl::blk::BLK_GETSIZE64;
use std::fs::File;
use std::io::Result;
use std::path::Path;

pub struct NativeBlockDevice {
    file: File,
}

impl NativeBlockDevice {
    pub fn open_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        File::open(path).map(|file| Self::from_file(file))
    }

    pub fn from_file(file: File) -> Self {
        Self { file }
    }
}

impl BlockDevice for NativeBlockDevice {
    fn size(&mut self) -> Result<u64> {
        BLK_GETSIZE64.ioctl(&mut self.file).map(|(_, size)| size)
    }
}
