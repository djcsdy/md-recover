use derive_more::{Add, AddAssign, Display, From, Into, Sub, SubAssign};

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
)]
#[display("{_0} bytes")]
pub struct BlockSize(pub u32);

impl Default for BlockSize {
    fn default() -> Self {
        Self(4096)
    }
}

impl From<BlockSize> for u64 {
    fn from(value: BlockSize) -> Self {
        u64::from(u32::from(value))
    }
}

impl From<BlockSize> for usize {
    fn from(value: BlockSize) -> Self {
        usize::try_from(u32::from(value)).unwrap()
    }
}
