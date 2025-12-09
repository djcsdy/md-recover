use crate::md::algorithm::MdAlgorithm;
use crate::md::units::{DeviceCount, SectorCount};
use crate::md::MdDeviceSuperblock;

#[derive(PartialEq, Clone, Hash, Debug)]
pub struct MdFormat {
    pub algorithm: MdAlgorithm,
    pub device_count: DeviceCount,
    pub sectors_per_device: SectorCount<u64>,
    pub chunk_size: SectorCount<u32>,
}

impl MdFormat {
    pub fn from_superblock(superblock: &MdDeviceSuperblock) -> Option<Self> {
        superblock.as_option().map(|superblock| Self {
            algorithm: superblock.algorithm(),
            device_count: superblock.raid_device_count(),
            sectors_per_device: superblock.sectors_per_device(),
            chunk_size: superblock.chunk_size(),
        })
    }

    pub fn from_superblock_reshape_status(superblock: &MdDeviceSuperblock) -> Option<Self> {
        superblock.as_option().and_then(|superblock| {
            superblock.reshape_status().and_then(|status| {
                Some(Self {
                    algorithm: status.new_algorithm,
                    device_count: superblock
                        .raid_device_count()
                        .checked_add(status.delta_devices)?,
                    sectors_per_device: superblock.sectors_per_device(),
                    chunk_size: status.new_chunk_size,
                })
            })
        })
    }

    pub fn parity_device_count(&self) -> Option<DeviceCount> {
        self.algorithm.parity_device_count()
    }

    pub fn data_device_count(&self) -> Option<DeviceCount> {
        u32::from(self.device_count)
            .checked_sub(u32::from(self.parity_device_count()?))
            .map(DeviceCount)
    }

    pub fn data_sector_count(&self) -> Option<SectorCount<u64>> {
        u64::from(self.sectors_per_device)
            .checked_mul(u64::from(self.data_device_count()?))
            .map(SectorCount)
    }
}
