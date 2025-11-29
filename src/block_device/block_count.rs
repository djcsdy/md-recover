use crate::block_device::BlockSize;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Sub, SubAssign};

#[derive(
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Copy,
    Hash,
    Debug,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    From,
    Display,
    Deref,
    DerefMut,
)]
#[display("{_0} blocks")]
pub struct BlockCount(pub u64);

impl BlockCount {
    pub fn size_bytes(self, block_size: BlockSize) -> u64 {
        self.0 * u64::from(block_size)
    }
}
