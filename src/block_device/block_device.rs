use crate::block_device::{BlockCount, BlockNumber, BlockSize};
use std::io;
use std::io::{Read, Seek, SeekFrom};

pub trait BlockDevice: Read + Seek + Sized {
    fn block_size(&self) -> io::Result<BlockSize>;

    fn block_count(&self) -> io::Result<BlockCount> {
        Ok(BlockCount(self.size()? / u64::from(self.block_size()?)))
    }

    fn size(&self) -> io::Result<u64>;

    fn read_block(&mut self, block_number: BlockNumber, buf: &mut [u8]) -> io::Result<usize> {
        let block_size = self.block_size()?;
        if buf.len() < usize::from(block_size) {
            Err(io::ErrorKind::InvalidInput.into())
        } else {
            self.seek(SeekFrom::Start(block_number.byte_pos(block_size)))?;
            self.read_exact(&mut buf[..usize::from(block_size)])?;
            Ok(usize::from(block_size))
        }
    }

    fn try_clone(&self) -> io::Result<Self>;
}
