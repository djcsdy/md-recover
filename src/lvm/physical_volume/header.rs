use crate::lvm::physical_volume::location::DiskLocation;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
use std::io::Read;

pub struct LvmPhysicalVolumeHeader {
    device_size_bytes: u64,
    data_areas: Vec<DiskLocation<Vec<u8>>>,
    metadata_areas: Vec<DiskLocation<Vec<u8>>>,
}

impl LvmPhysicalVolumeHeader {
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let device_size_bytes = reader.read_u64::<LittleEndian>()?;
        let data_areas = DiskLocation::read_all(&mut reader)?;
        let metadata_areas = DiskLocation::read_all(&mut reader)?;

        Ok(Self {
            device_size_bytes,
            data_areas,
            metadata_areas,
        })
    }
}
