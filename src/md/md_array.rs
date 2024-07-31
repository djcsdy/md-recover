use crate::block_device::BlockDevice;
use crate::ext::MultiMap;
use crate::md::algorithm::MdAlgorithm;
use crate::md::device::MdDeviceId;
use crate::md::superblock::{ArrayUuid, ReshapeStatus, Superblock};
use crate::md::MdDevice;
use std::collections::HashMap;
use std::ffi::OsString;

pub struct MdArray {
    devices: Vec<MdDevice<Box<dyn Superblock>, Box<dyn BlockDevice>>>,
}

impl MdArray {
    pub fn new(devices: Vec<MdDevice<Box<dyn Superblock>, Box<dyn BlockDevice>>>) -> Self {
        Self { devices }
    }

    fn diagnose_array_uuid_problem(&self) -> Option<HashMap<ArrayUuid, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(
            self.devices
                .iter()
                .map(|device| (device.superblock.array_uuid(), device.id.clone())),
        );

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
                .array_name()
                .map(|array_name| (array_name.into(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_algorithm_problem(&self) -> Option<HashMap<MdAlgorithm, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(
            self.devices
                .iter()
                .map(|device| (device.superblock.algorithm().clone(), device.id.clone())),
        );

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_size_problem(&self) -> Option<HashMap<u64, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(
            self.devices
                .iter()
                .map(|device| (device.superblock.size(), device.id.clone())),
        );

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_chunk_size_problem(&self) -> Option<HashMap<u32, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(
            self.devices
                .iter()
                .map(|device| (device.superblock.chunk_size(), device.id.clone())),
        );

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_disk_count_problem(&self) -> Option<HashMap<u32, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(
            self.devices
                .iter()
                .map(|device| (device.superblock.raid_disks(), device.id.clone())),
        );

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_reshape_problem(&self) -> Option<HashMap<ReshapeStatus, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(
            self.devices
                .iter()
                .map(|device| (device.superblock.reshape_status(), device.id.clone())),
        );

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_event_count_problem(&self) -> Option<HashMap<u64, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(
            self.devices
                .iter()
                .map(|device| (device.superblock.event_count(), device.id.clone())),
        );

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_device_roles_problem(&self) -> Option<HashMap<Vec<u16>, Vec<MdDeviceId>>> {
        let map = HashMap::from_multi_iter(
            self.devices
                .iter()
                .map(|device| (device.superblock.device_roles(), device.id.clone())),
        );

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }
}
