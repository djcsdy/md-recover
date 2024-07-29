#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct ReshapeStatus {
    pub new_level: u32,
    pub reshape_position: u64,
    pub delta_disks: u32,
    pub new_layout: u32,
    pub new_chunk_size: u32,
    pub new_offset: u32,
}
