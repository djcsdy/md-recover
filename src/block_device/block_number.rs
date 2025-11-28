use crate::block_device::BlockSize;
use derive_more::{Deref, DerefMut, Display, From, Into};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display, Deref, DerefMut,
)]
#[display("block #{_0}")]
pub struct BlockNumber(pub u64);

impl BlockNumber {
    pub(in crate::block_device) fn byte_pos(self, block_size: BlockSize) -> u64 {
        u64::from(self) * u64::from(block_size)
    }
}
