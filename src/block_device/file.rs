use crate::block_device::internal::InternalFile;
use crate::block_device::{BlockCount, BlockDevice, BlockNumber, BlockSize};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

#[derive(Debug)]
pub struct FileBlockDevice {
    file: InternalFile,
    block_size: BlockSize,
}

impl FileBlockDevice {
    pub fn open_path<P: AsRef<Path>>(path: P, block_size: BlockSize) -> io::Result<Self> {
        Ok(Self::from_file(File::open(path)?, block_size))
    }

    pub fn from_file(file: File, block_size: BlockSize) -> Self {
        Self {
            file: InternalFile(file),
            block_size,
        }
    }
}

impl BlockDevice for FileBlockDevice {
    fn block_size(&self) -> io::Result<BlockSize> {
        Ok(self.block_size)
    }

    fn block_count(&self) -> io::Result<BlockCount> {
        Ok(BlockCount(
            self.file
                .as_ref()
                .metadata()
                .map(|metadata| metadata.size())?
                / u64::from(self.block_size()?),
        ))
    }

    fn read_block(&mut self, block_number: BlockNumber, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read_block(block_number, self.block_size()?, buf)
    }

    fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            file: self.file.try_clone()?,
            block_size: self.block_size,
        })
    }
}

impl Read for FileBlockDevice {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.as_mut().read(buf)
    }
}

impl Seek for FileBlockDevice {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.file.as_mut().seek(pos)
    }
}
