use crate::block_device::BlockDevice;
use crate::ext::MultiMap;
use crate::md::device::MdDeviceId;
use crate::md::superblock::{ArrayUuid, Superblock};
use crate::md::MdDevice;
use std::collections::HashMap;

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
}
