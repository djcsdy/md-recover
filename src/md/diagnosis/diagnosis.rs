use crate::md::algorithm::MdAlgorithm;
use crate::md::device::MdDeviceId;
use crate::md::superblock::ArrayUuid;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Diagnosis {
    missing_superblock_problem: Option<HashSet<MdDeviceId>>,
    array_uuid_problem: Option<HashMap<ArrayUuid, Vec<MdDeviceId>>>,
    array_name_problem: Option<HashMap<OsString, Vec<MdDeviceId>>>,
    algorithm_problem: Option<HashMap<MdAlgorithm, Vec<MdDeviceId>>>,
    size_problem: Option<HashMap<u64, Vec<MdDeviceId>>>,
    chunk_size_problem: Option<HashMap<u32, Vec<MdDeviceId>>>,
    disk_count_problem: Option<HashMap<u32, Vec<MdDeviceId>>>,
    reshape_problem: Option<()>, // TODO
    device_number_problem: Option<HashMap<u32, Vec<MdDeviceId>>>,
    events_problem: Option<HashMap<u64, Vec<MdDeviceId>>>,
    device_roles_problem: Option<()>, // TODO
}
