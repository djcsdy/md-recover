use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};
use std::path::Path;

use crate::ioctl::blk::BLK_GETSIZE64;
use crate::md::superblock::Superblock;

use super::superblock::SuperblockVersion1;

pub struct MdDevice<S: Superblock, R: Read> {
    pub superblock: S,
    pub minor_version: u32,
    reader: R,
}

impl<S: Superblock, R: Read> MdDevice<S, R> {
    const MIN_DEVICE_SIZE: u64 = 12288;
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
        Err(Error::from(ErrorKind::InvalidData))
    }
}
