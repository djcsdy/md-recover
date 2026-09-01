mod header;
mod label;
mod location;

use crate::block_device::BlockDevice;
use crate::lvm::physical_volume::header::PhysicalVolumeHeader;
use crate::lvm::physical_volume::label::LvmLabel;
use std::io;
use std::io::{Read, Seek};

#[derive(Clone)]
pub struct PhysicalVolume<D>
where
    D: BlockDevice + Read + Seek,
{
    header: PhysicalVolumeHeader,
    device: D,
}

impl<D> PhysicalVolume<D>
where
    D: BlockDevice + Read + Seek,
{
    pub fn open(mut device: D) -> io::Result<Self> {
        let label = LvmLabel::scan_read(&mut device)?;

        if label.label_type() != b"LVM2 001" {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let header = PhysicalVolumeHeader::read(label.header().as_slice())?;

        Ok(Self { header, device })
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        self.device.try_clone().map(|device| Self {
            header: self.header.clone(),
            device,
        })
    }
}
