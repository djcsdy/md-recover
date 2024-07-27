use std::io::{Read, Result, Seek};

pub trait BlockDevice: Read + Seek {
    fn size(&mut self) -> Result<u64>;
}

impl BlockDevice for Box<dyn BlockDevice> {
    fn size(&mut self) -> Result<u64> {
        (**self).size()
    }
}
