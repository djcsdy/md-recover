use binary_layout::prelude::*;

use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;
pub use layout::NestedView as NestedReshapeStatusVersion0;
pub use layout::View as ReshapeStatusVersion0;

define_layout!(layout, BigEndian, {
    reshape_position: u64,
    new_level: u32,
    delta_disks: u32,
    new_layout: u32,
    new_chunk_size: u32
});

impl<S: AsRef<[u8]>> From<ReshapeStatusVersion0<S>> for ReshapeStatus {
    fn from(value: ReshapeStatusVersion0<S>) -> Self {
        Self {
            new_algorithm: MdAlgorithm::from_level_and_layout(
                value.new_level().read(),
                value.new_layout().read(),
            ),
            reshape_position: value.reshape_position().read(),
            delta_disks: value.delta_disks().read(),
            new_chunk_size: value.new_chunk_size().read(),
            new_offset: 0,
        }
    }
}
