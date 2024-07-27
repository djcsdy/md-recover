use crate::block_device::BlockDevice;
use crate::ioctl::blk::BLK_GETSIZE64;
use crate::md::superblock::Superblock;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};
use std::path::Path;

use super::superblock::{SuperblockVersion0, SuperblockVersion1};

pub struct MdDevice<S: Superblock, R: Read + Seek> {
    pub superblock: S,
    pub minor_version: u32,
    reader: R,
}

impl<S: Superblock, R: Read + Seek> MdDevice<S, R> {
    const MIN_DEVICE_SIZE: u64 = 12288;
    const MIN_SUPERBLOCK_0_DEVICE_SIZE: u64 = 65536;
}

impl MdDevice<Box<dyn Superblock>, File> {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let (_, size) = BLK_GETSIZE64.ioctl(&file)?;

        if size < Self::MIN_DEVICE_SIZE {
            return Err(Error::from(ErrorKind::Other));
        }

        for (minor_version, offset) in [(2, 8 << 9), (1, 0), (0, (((size >> 9) - 16) & !7) << 9)] {
            file.seek(SeekFrom::Start(offset))?;
            match SuperblockVersion1::read(&mut file) {
                Ok(superblock) => {
                    return Ok(Self {
                        superblock: Box::new(superblock),
                        minor_version,
                        reader: file,
                    });
                }
                Err(_) => {}
            }
        }

        if size >= Self::MIN_SUPERBLOCK_0_DEVICE_SIZE {
            file.seek(SeekFrom::Start((size & !65535) - 65536))?;
            let superblock = SuperblockVersion0::read(&mut file)?;
            let minor_version = superblock.minor_version();

            Ok(Self {
                superblock: Box::new(superblock),
                minor_version,
                reader: file,
            })
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}

impl<S: Superblock, R: Read + Seek> BlockDevice for MdDevice<S, R> {
    fn size(&mut self) -> Result<u64> {
        todo!()
    }
}

impl<S: Superblock, R: Read + Seek> Read for MdDevice<S, R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
    }
}

impl<S: Superblock, R: Read + Seek> Seek for MdDevice<S, R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.reader.seek(pos)
    }
}
