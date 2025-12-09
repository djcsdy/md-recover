use binary_layout::prelude::*;

use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;
use crate::md::units::{DeviceCount, SectorNumber};

#[allow(unused_imports)]
pub use self::{
    layout::NestedView as NestedReshapeStatusVersion1, layout::View as ReshapeStatusVersion1,
};

binary_layout!(layout, LittleEndian, {
    new_level: u32,
    reshape_position: SectorNumber as u64,
    delta_devices: DeviceCount as u32,
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
            delta_devices: value.delta_devices().read(),
            new_chunk_size: value.new_chunk_size().read(),
            new_offset: value.new_offset().read(),
        }
    }
}
