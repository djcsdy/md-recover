use crate::block_device::BlockDevice;
use crate::ext::MultiMap;
use crate::md::algorithm::MdAlgorithm;
use crate::md::device::MdDeviceId;
use crate::md::superblock::{ArrayUuid, Superblock};
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
}
