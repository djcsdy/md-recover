mod header;
mod label;
mod location;

use crate::block_device::BlockDevice;
use std::io;
use std::io::{Read, Seek};

#[derive(Clone)]
pub struct LvmPhysicalVolume<D>
where
    D: BlockDevice + Read + Seek,
{
    device: D,
}

impl<D> LvmPhysicalVolume<D>
where
    D: BlockDevice + Read + Seek,
{
    pub fn open(device: D) -> io::Result<Self> {
        Ok(Self { device })
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        self.device.try_clone().map(|device| Self { device })
    }
}
