use binary_layout::prelude::*;

use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;
pub use layout::NestedView as NestedReshapeStatusVersion1;
pub use layout::View as ReshapeStatusVersion1;

binary_layout!(layout, LittleEndian, {
    new_level: u32,
    reshape_position: u64,
    delta_disks: u32,
    new_layout: u32,
    new_chunk_size: u32,
    new_offset: u32
});

impl<S: AsRef<[u8]>> From<ReshapeStatusVersion1<S>> for ReshapeStatus {
    fn from(value: ReshapeStatusVersion1<S>) -> Self {
        Self {
            new_algorithm: MdAlgorithm::from_level_and_layout(
                value.new_level().read(),
                value.new_layout().read(),
            ),
            reshape_position: value.reshape_position().read(),
            delta_disks: value.delta_disks().read(),
            new_chunk_size: value.new_chunk_size().read(),
            new_offset: value.new_offset().read(),
        }
    }
}
