use crate::md::algorithm::MdAlgorithm;
use crate::md::device::MdDeviceId;
use crate::md::superblock::{ArrayUuid, ReshapeStatus};
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Diagnosis {
    pub missing_superblock_problem: Option<HashSet<MdDeviceId>>,
    pub array_uuid_problem: Option<HashMap<ArrayUuid, Vec<MdDeviceId>>>,
    pub array_name_problem: Option<HashMap<OsString, Vec<MdDeviceId>>>,
    pub algorithm_problem: Option<HashMap<MdAlgorithm, Vec<MdDeviceId>>>,
    pub size_problem: Option<HashMap<u64, Vec<MdDeviceId>>>,
    pub chunk_size_problem: Option<HashMap<u32, Vec<MdDeviceId>>>,
    pub disk_count_problem: Option<HashMap<u32, Vec<MdDeviceId>>>,
    pub reshape_problem: Option<HashMap<ReshapeStatus, Vec<MdDeviceId>>>,
    pub event_count_problem: Option<HashMap<u64, Vec<MdDeviceId>>>,
    pub device_roles_problem: Option<HashMap<Vec<u16>, Vec<MdDeviceId>>>,
}
