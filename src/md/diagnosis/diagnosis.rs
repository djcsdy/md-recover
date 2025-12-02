use crate::md::algorithm::MdAlgorithm;
use crate::md::device::MdDeviceId;
use crate::md::superblock::{ArrayUuid, MdDeviceRole, ReshapeStatus};
use crate::md::units::{DeviceCount, SectorCount};
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Diagnosis {
    pub device_too_small_problem: Option<HashSet<Rc<MdDeviceId>>>,
    pub missing_superblock_problem: Option<HashSet<Rc<MdDeviceId>>>,
    pub array_uuid_problem: Option<HashMap<ArrayUuid, Vec<Rc<MdDeviceId>>>>,
    pub array_name_problem: Option<HashMap<OsString, Vec<Rc<MdDeviceId>>>>,
    pub algorithm_problem: Option<HashMap<MdAlgorithm, Vec<Rc<MdDeviceId>>>>,
    pub size_problem: Option<HashMap<SectorCount<u64>, Vec<Rc<MdDeviceId>>>>,
    pub chunk_size_problem: Option<HashMap<SectorCount<u32>, Vec<Rc<MdDeviceId>>>>,
    pub disk_count_problem: Option<HashMap<DeviceCount, Vec<Rc<MdDeviceId>>>>,
    pub reshape_problem: Option<HashMap<Option<ReshapeStatus>, Vec<Rc<MdDeviceId>>>>,
    pub event_count_problem: Option<HashMap<u64, Vec<Rc<MdDeviceId>>>>,
    pub device_roles_problem: Option<HashMap<Vec<MdDeviceRole>, Vec<Rc<MdDeviceId>>>>,
}
