use crate::block_device::BlockDevice;
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
        let mut map = HashMap::new();
        for device in &self.devices {
            let array_uuid = device.superblock.array_uuid();
            match map.get_mut(&array_uuid) {
                None => {
                    map.insert(array_uuid, vec![device.id.clone()]);
                }
                Some(devices) => {
                    devices.push(device.id.clone());
                }
            }
        }

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }
}
