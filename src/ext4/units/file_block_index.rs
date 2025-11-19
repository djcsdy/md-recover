use binary_layout::LayoutAs;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Into, Sub, SubAssign};
use std::convert::Infallible;

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
#[display("file block #{_0}")]
pub struct FileBlockIndex(pub u32);

impl LayoutAs<u32> for FileBlockIndex {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.0)
    }
}
