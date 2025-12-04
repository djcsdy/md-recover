use crate::md::algorithm::MdAlgorithm;
use crate::md::units::{DeviceCount, SectorCount};
use crate::md::MdDeviceSuperblock;

#[derive(PartialEq, Clone, Hash, Debug)]
pub struct MdConfig {
    pub algorithm: MdAlgorithm,
    pub device_count: DeviceCount,
    pub sectors_per_device: SectorCount<u64>,
}

impl MdConfig {
    pub(crate) fn from_superblocks<I, S>(superblocks: I) -> Option<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<MdDeviceSuperblock>,
    {
        superblocks
            .into_iter()
            .map(|superblock| Self::from_superblock(superblock.as_ref()))
            .reduce(|a, b| if a == b { a } else { None })
            .flatten()
    }

    fn from_superblock(superblock: &MdDeviceSuperblock) -> Option<Self> {
        superblock.as_option().map(|superblock| Self {
            algorithm: superblock.algorithm(),
            device_count: superblock.raid_device_count(),
            sectors_per_device: superblock.sectors_per_device(),
        })
    }
}
