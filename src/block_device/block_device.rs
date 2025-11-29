use crate::block_device::{BlockCount, BlockNumber, BlockSize};
use std::io;
use std::io::{Read, Seek};

pub trait BlockDevice: Read + Seek + Sized {
    fn block_size(&self) -> io::Result<BlockSize>;

    fn block_count(&self) -> io::Result<BlockCount>;

    fn read_block(&mut self, block_number: BlockNumber, buf: &mut [u8]) -> io::Result<usize>;

    fn try_clone(&self) -> io::Result<Self>;
}
