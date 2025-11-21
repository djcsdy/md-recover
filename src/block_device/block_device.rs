use std::io::{Read, Result, Seek};

pub trait BlockDevice: Read + Seek {
    fn size(&self) -> Result<u64>;
}
