use crate::block_device::BlockDevice;
use crate::ext::MultiMap;
use crate::md::algorithm::MdAlgorithm;
use crate::md::device::{MdDeviceId, MdDeviceSuperblock};
use crate::md::diagnosis::Diagnosis;
use crate::md::superblock::{ArrayUuid, ReshapeStatus, Superblock};
use crate::md::MdDevice;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;

pub struct MdArray {
    devices: Vec<MdDevice<Box<dyn Superblock>, Box<dyn BlockDevice>>>,
}

impl MdArray {
    pub fn new(devices: Vec<MdDevice<Box<dyn Superblock>, Box<dyn BlockDevice>>>) -> Self {
        Self { devices }
    }

    pub fn diagnose(&self) -> Diagnosis {
        Diagnosis {
            device_too_small_problem: self.diagnose_device_too_small_problem(),
            missing_superblock_problem: self.diagnose_missing_superblock_problem(),
            array_uuid_problem: self.diagnose_array_uuid_problem(),
            array_name_problem: self.diagnose_array_name_problem(),
            algorithm_problem: self.diagnose_algorithm_problem(),
            size_problem: self.diagnose_size_problem(),
            chunk_size_problem: self.diagnose_chunk_size_problem(),
            disk_count_problem: self.diagnose_disk_count_problem(),
            reshape_problem: self.diagnose_reshape_problem(),
            event_count_problem: self.diagnose_event_count_problem(),
            device_roles_problem: self.diagnose_device_roles_problem(),
        }
    }

    fn diagnose_device_too_small_problem(&self) -> Option<HashSet<MdDeviceId>> {
        let set = HashSet::from_iter(self.devices.iter().filter_map(
            |device| match device.superblock {
                MdDeviceSuperblock::TooSmall => Some(device.id.clone()),
                _ => None,
            },
        ));

        if set.len() > 0 {
            Some(set)
        } else {
            None
        }
    }

    fn diagnose_missing_superblock_problem(&self) -> Option<HashSet<MdDeviceId>> {
        let set = HashSet::from_iter(self.devices.iter().filter_map(
            |device| match device.superblock {
                MdDeviceSuperblock::Missing => Some(device.id.clone()),
                _ => None,
            },
        ));

        if set.len() > 0 {
            Some(set)
        } else {
            None
        }
    }

    fn diagnose_array_uuid_problem(&self) -> Option<HashMap<ArrayUuid, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.array_uuid(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_array_name_problem(&self) -> Option<HashMap<OsString, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .and_then(|superblock| superblock.array_name())
                .map(|array_name| (array_name.into(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_algorithm_problem(&self) -> Option<HashMap<MdAlgorithm, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.algorithm().clone(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_size_problem(&self) -> Option<HashMap<u64, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.size(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_chunk_size_problem(&self) -> Option<HashMap<u32, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.chunk_size(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_disk_count_problem(&self) -> Option<HashMap<u32, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.raid_disks(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_reshape_problem(&self) -> Option<HashMap<ReshapeStatus, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.reshape_status(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_event_count_problem(&self) -> Option<HashMap<u64, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.event_count(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_device_roles_problem(&self) -> Option<HashMap<Vec<u16>, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.device_roles(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }
}
