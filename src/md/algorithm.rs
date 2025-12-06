use crate::md::raid5::Raid5Algorithm;
use crate::md::raid6::Raid6Algorithm;
use crate::md::units::{DeviceCount, DeviceNumber, SectorCount, SectorNumber};
use std::io;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum MdAlgorithm {
    Unsupported { level: u32, layout: u32 },
    Raid5(Raid5Algorithm),
    Raid6(Raid6Algorithm),
}

impl MdAlgorithm {
    pub fn from_level_and_layout(level: u32, layout: u32) -> Self {
        match level {
            5 => Raid5Algorithm::from_layout(layout).map(Self::Raid5),
            6 => Raid6Algorithm::from_layout(layout).map(Self::Raid6),
            _ => None,
        }
        .unwrap_or(Self::Unsupported { level, layout })
    }

    pub fn parity_device_count(&self) -> Option<DeviceCount> {
        match self {
            MdAlgorithm::Unsupported { .. } => None,
            MdAlgorithm::Raid5(_) => Some(DeviceCount(1)),
            MdAlgorithm::Raid6(_) => Some(DeviceCount(2)),
        }
    }

    pub(in crate::md) fn read_sector<F>(
        &self,
        sector_number: SectorNumber,
        sectors_per_chunk: SectorCount<u32>,
        raid_device_count: DeviceCount,
        read_sector_of_device: F,
    ) -> io::Result<Vec<u8>>
    where
        F: FnMut(DeviceNumber, SectorNumber, &mut [u8]) -> io::Result<usize>,
    {
        match self {
            MdAlgorithm::Raid6(algorithm) => algorithm.read_sector(
                sector_number,
                sectors_per_chunk,
                raid_device_count,
                read_sector_of_device,
            ),
            _ => Err(io::ErrorKind::Unsupported)?,
        }
    }
}
