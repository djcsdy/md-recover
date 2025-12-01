use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Sub, SubAssign};
use std::ops::{Add, Sub};

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
pub struct BlockCount<C: PartialOrd + Add + Sub>(pub C);

impl From<BlockCount<u16>> for u16 {
    fn from(value: BlockCount<u16>) -> Self {
        value.0
    }
}

impl From<BlockCount<u32>> for u32 {
    fn from(value: BlockCount<u32>) -> Self {
        value.0
    }
}

impl From<BlockCount<u64>> for u64 {
    fn from(value: BlockCount<u64>) -> Self {
        value.0
    }
}

impl From<BlockCount<u16>> for BlockCount<u32> {
    fn from(value: BlockCount<u16>) -> Self {
        Self(value.0.into())
    }
}

impl From<BlockCount<u16>> for BlockCount<u64> {
    fn from(value: BlockCount<u16>) -> Self {
        Self(value.0.into())
    }
}

impl From<BlockCount<u32>> for BlockCount<u64> {
    fn from(value: BlockCount<u32>) -> Self {
        Self(value.0.into())
    }
}
