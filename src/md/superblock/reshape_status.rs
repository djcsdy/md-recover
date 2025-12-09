use crate::md::algorithm::MdAlgorithm;
use crate::md::units::{DeviceCount, SectorCount, SectorNumber};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct ReshapeStatus {
    pub new_algorithm: MdAlgorithm,
    pub reshape_position: SectorNumber,
    pub delta_devices: DeviceCount,
    pub new_chunk_size: SectorCount<u32>,
    pub new_offset: u32,
}
