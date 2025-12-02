use std::ffi::OsStr;

use super::{ArrayUuid, MdDeviceRole};
use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;
use crate::md::units::{DeviceCount, MetadataEventCount, SectorCount};

pub trait Superblock {
    fn valid(&self) -> bool;
    fn major_version(&self) -> u32;
    fn minor_version(&self) -> u32;
    fn array_uuid(&self) -> ArrayUuid;
    fn array_name(&self) -> Option<&OsStr>;
    fn algorithm(&self) -> MdAlgorithm;
    fn sectors_per_device(&self) -> SectorCount<u64>;
    fn chunk_size(&self) -> SectorCount<u32>;
    fn raid_disks(&self) -> DeviceCount;
    fn reshape_status(&self) -> Option<ReshapeStatus>;
    fn event_count(&self) -> MetadataEventCount;
    fn device_roles(&self) -> Vec<MdDeviceRole>;
}

impl Superblock for Box<dyn Superblock> {
    fn valid(&self) -> bool {
        (**self).valid()
    }

    fn major_version(&self) -> u32 {
        (**self).major_version()
    }

    fn minor_version(&self) -> u32 {
        (**self).minor_version()
    }

    fn array_uuid(&self) -> ArrayUuid {
        (**self).array_uuid()
    }

    fn array_name(&self) -> Option<&OsStr> {
        (**self).array_name()
    }

    fn algorithm(&self) -> MdAlgorithm {
        (**self).algorithm()
    }

    fn sectors_per_device(&self) -> SectorCount<u64> {
        (**self).sectors_per_device()
    }

    fn chunk_size(&self) -> SectorCount<u32> {
        (**self).chunk_size()
    }

    fn raid_disks(&self) -> DeviceCount {
        (**self).raid_disks()
    }

    fn reshape_status(&self) -> Option<ReshapeStatus> {
        (**self).reshape_status()
    }

    fn event_count(&self) -> MetadataEventCount {
        (**self).event_count()
    }

    fn device_roles(&self) -> Vec<MdDeviceRole> {
        (**self).device_roles()
    }
}
