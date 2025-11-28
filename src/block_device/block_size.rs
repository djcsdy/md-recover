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
#[display("{_0} bytes")]
pub struct BlockSize(pub u32);

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
