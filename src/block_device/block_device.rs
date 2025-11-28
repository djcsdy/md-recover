use std::io;
use std::io::{Read, Seek};

pub trait BlockDevice: Read + Seek + Sized {
    fn block_size(&self) -> io::Result<usize>;
    fn size(&self) -> io::Result<u64>;
    fn try_clone(&self) -> io::Result<Self>;
}
