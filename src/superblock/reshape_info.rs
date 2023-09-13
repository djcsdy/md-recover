use binary_layout::prelude::*;

define_layout!(layout, LittleEndian, {
    new_level: u32,
    reshape_position: u64,
    delta_disks: u32,
    new_layout: u32,
    new_chunk_size: u32,
    new_offset: u32
});

pub use layout::View as ReshapeInfo;

impl<B: AsRef<[u8]>> ReshapeInfo<B> {
    pub const LENGTH: usize = 28;
}
