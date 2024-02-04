use binary_layout::prelude::*;

pub use layout::NestedView as NestedReshapeInfo;

define_layout!(layout, LittleEndian, {
    new_level: u32,
    reshape_position: u64,
    delta_disks: u32,
    new_layout: u32,
    new_chunk_size: u32,
    new_offset: u32
});
