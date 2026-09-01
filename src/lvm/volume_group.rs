use crate::block_device::BlockDevice;
use crate::lvm::LvmPhysicalVolume;
use itertools::Itertools;
use std::io;
use std::io::{Read, Seek};

#[derive(Clone)]
pub struct VolumeGroup<D>
where
    D: BlockDevice + Read + Seek,
{
    physical_volumes: Vec<LvmPhysicalVolume<D>>,
}

impl<D> VolumeGroup<D>
where
    D: BlockDevice + Read + Seek,
{
    pub fn open(physical_volumes: Vec<LvmPhysicalVolume<D>>) -> Self {
        Self { physical_volumes }
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        self.physical_volumes
            .iter()
            .map(|physical_volume| physical_volume.try_clone())
            .try_collect()
            .map(|physical_volumes| Self { physical_volumes })
    }
}
