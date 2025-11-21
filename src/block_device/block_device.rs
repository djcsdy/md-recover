use std::io::{Read, Result, Seek};

pub trait BlockDevice: Read + Seek + Sized {
    fn size(&self) -> Result<u64>;
    fn try_clone(&self) -> Result<Self>;
}
