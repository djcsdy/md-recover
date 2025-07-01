use crate::block_device::BlockDevice;
use crate::ext4::superblock::Superblock;
use std::io::{Result, SeekFrom};

pub struct Ext4Fs<D: BlockDevice> {
    device: D,
    superblock: Superblock<Vec<u8>>,
}

impl<D: BlockDevice> Ext4Fs<D> {
    pub fn open(mut device: D) -> Result<Self> {
        device.seek(SeekFrom::Start(1024))?;
        let superblock = Superblock::read(&mut device)?;
        Ok(Self { device, superblock })
    }
}
