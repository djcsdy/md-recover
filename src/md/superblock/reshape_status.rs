use crate::md::algorithm::MdAlgorithm;
use crate::md::units::SectorNumber;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct ReshapeStatus {
    pub new_algorithm: MdAlgorithm,
    pub reshape_position: SectorNumber,
    pub delta_disks: u32,
    pub new_chunk_size: u32,
    pub new_offset: u32,
}
