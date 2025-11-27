use std::io;
use std::io::{Read, Seek};

pub trait BlockDevice: Read + Seek + Sized {
    fn size(&self) -> io::Result<u64>;
    fn try_clone(&self) -> io::Result<Self>;
}
