use binary_layout::prelude::*;

pub use layout::NestedView as NestedReshapeStatusVersion0;
pub use layout::View as ReshapeStatusVersion0;

define_layout!(layout, LittleEndian, {
    reshape_position: u64,
    new_level: u32,
    delta_disks: u32,
    new_layout: u32,
    new_chunk_size: u32
});
