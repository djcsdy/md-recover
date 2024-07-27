use crate::block_device::BlockDevice;
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

impl BlockDevice for NativeBlockDevice {}
