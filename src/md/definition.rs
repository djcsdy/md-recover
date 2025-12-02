use crate::block_device::BlockDevice;
use crate::ext::MultiMap;
use crate::md::algorithm::MdAlgorithm;
use crate::md::diagnosis::Diagnosis;
use crate::md::superblock::{ArrayUuid, MdDeviceRole, ReshapeStatus};
use crate::md::units::{DeviceCount, MetadataEventCount, SectorCount};
use crate::md::{MdDevice, MdDeviceId, MdDeviceSuperblock};
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::io::{Read, Seek};
use std::rc::Rc;

pub struct MdArrayDefinition<D>
where
    D: BlockDevice + Read + Seek,
{
    pub devices: Vec<Rc<MdDevice<D>>>,
}

impl<D> MdArrayDefinition<D>
where
    D: BlockDevice + Read + Seek,
{
    pub fn diagnose(&self) -> Diagnosis {
        Diagnosis {
            device_too_small_problem: self.diagnose_device_too_small_problem(),
            missing_superblock_problem: self.diagnose_missing_superblock_problem(),
            array_uuid_problem: self.diagnose_array_uuid_problem(),
            array_name_problem: self.diagnose_array_name_problem(),
            algorithm_problem: self.diagnose_algorithm_problem(),
            size_problem: self.diagnose_size_problem(),
            chunk_size_problem: self.diagnose_chunk_size_problem(),
            device_count_problem: self.diagnose_device_count_problem(),
            reshape_problem: self.diagnose_reshape_problem(),
            event_count_problem: self.diagnose_event_count_problem(),
            device_roles_problem: self.diagnose_device_roles_problem(),
        }
    }

    fn diagnose_device_too_small_problem(&self) -> Option<HashSet<Rc<MdDeviceId>>> {
        let set = HashSet::from_iter(self.devices.iter().filter_map(|device| {
            match device.superblock.as_ref() {
                MdDeviceSuperblock::TooSmall => Some(device.id.clone()),
                _ => None,
            }
        }));

        if set.is_empty() {
            None
        } else {
            Some(set)
        }
    }

    fn diagnose_missing_superblock_problem(&self) -> Option<HashSet<Rc<MdDeviceId>>> {
        let set = HashSet::from_iter(self.devices.iter().filter_map(|device| {
            match device.superblock.as_ref() {
                MdDeviceSuperblock::Missing => Some(device.id.clone()),
                _ => None,
            }
        }));

        if set.is_empty() {
            None
        } else {
            Some(set)
        }
    }

    fn diagnose_array_uuid_problem(&self) -> Option<HashMap<ArrayUuid, Vec<Rc<MdDeviceId>>>> {
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

    fn diagnose_array_name_problem(&self) -> Option<HashMap<OsString, Vec<Rc<MdDeviceId>>>> {
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

    fn diagnose_algorithm_problem(&self) -> Option<HashMap<MdAlgorithm, Vec<Rc<MdDeviceId>>>> {
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

    fn diagnose_size_problem(&self) -> Option<HashMap<SectorCount<u64>, Vec<Rc<MdDeviceId>>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.sectors_per_device(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_chunk_size_problem(
        &self,
    ) -> Option<HashMap<SectorCount<u32>, Vec<Rc<MdDeviceId>>>> {
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

    fn diagnose_device_count_problem(&self) -> Option<HashMap<DeviceCount, Vec<Rc<MdDeviceId>>>> {
        let map = HashMap::from_multi_iter(self.devices.iter().filter_map(|device| {
            device
                .superblock
                .as_option()
                .map(|superblock| (superblock.raid_device_count(), device.id.clone()))
        }));

        if map.len() > 1 {
            Some(map)
        } else {
            None
        }
    }

    fn diagnose_reshape_problem(
        &self,
    ) -> Option<HashMap<Option<ReshapeStatus>, Vec<Rc<MdDeviceId>>>> {
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

    fn diagnose_event_count_problem(
        &self,
    ) -> Option<HashMap<MetadataEventCount, Vec<Rc<MdDeviceId>>>> {
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

    fn diagnose_device_roles_problem(
        &self,
    ) -> Option<HashMap<Vec<MdDeviceRole>, Vec<Rc<MdDeviceId>>>> {
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
