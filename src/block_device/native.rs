use crate::block_device::internal::InternalFile;
use crate::block_device::{BlockDevice, BlockSize};
use crate::ioctl::blk::{BLK_GETSIZE64, BLK_PBSZGET};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct NativeBlockDevice {
    file: InternalFile,
}

impl NativeBlockDevice {
    pub fn open_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        File::open(path).map(Self::from_file)
    }

    pub fn from_file(file: File) -> Self {
        Self {
            file: InternalFile(file),
        }
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            file: self.file.try_clone()?,
        })
    }
}

impl BlockDevice for NativeBlockDevice {
    fn block_size(&self) -> io::Result<BlockSize> {
        BLK_PBSZGET
            .ioctl(self.file.as_ref())
            .map(|(_, size)| BlockSize(size))
    }

    fn size(&self) -> io::Result<u64> {
        BLK_GETSIZE64
            .ioctl(self.file.as_ref())
            .map(|(_, size)| size)
    }

    fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            file: self.file.try_clone()?,
        })
    }
}

impl Read for NativeBlockDevice {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.as_mut().read(buf)
    }
}

impl Seek for NativeBlockDevice {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.file.as_mut().seek(pos)
    }
}
