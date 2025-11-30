use crate::block_device::BlockSize;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Into, Sub, SubAssign};

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
    Into,
    Display,
    Deref,
    DerefMut,
)]
#[display("{_0} blocks")]
pub struct BlockCount(pub u64);

impl BlockCount {
    pub fn size_bytes(self, block_size: BlockSize) -> Option<u64> {
        self.0.checked_mul(u64::from(block_size))
    }
}
