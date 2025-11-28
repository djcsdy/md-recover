use crate::block_device::BlockCount;
use std::io;
use std::io::{Read, Seek};

pub trait BlockDevice: Read + Seek + Sized {
    fn block_size(&self) -> io::Result<usize>;
    fn block_count(&self) -> io::Result<BlockCount> {
        Ok(BlockCount(
            self.size()?
                / u64::try_from(self.block_size()?)
                    .or(Err(io::Error::from(io::ErrorKind::InvalidData)))?,
        ))
    }
    fn size(&self) -> io::Result<u64>;
    fn try_clone(&self) -> io::Result<Self>;
}
