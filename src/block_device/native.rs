use crate::block_device::internal::InternalFile;
use crate::block_device::{BlockCount, BlockDevice, BlockNumber, BlockSize};
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
}

impl BlockDevice for NativeBlockDevice {
    fn block_size(&self) -> io::Result<BlockSize> {
        BLK_PBSZGET
            .ioctl(self.file.as_ref())
            .map(|(_, size)| BlockSize(size))
    }

    fn block_count(&self) -> io::Result<BlockCount> {
        Ok(BlockCount(
            BLK_GETSIZE64.ioctl(self.file.as_ref())?.1 / u64::from(self.block_size()?),
        ))
    }

    fn read_block(&mut self, block_number: BlockNumber, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read_block(block_number, self.block_size()?, buf)
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
