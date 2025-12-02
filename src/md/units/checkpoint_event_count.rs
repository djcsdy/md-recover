use binary_layout::LayoutAs;
use derive_more::{Add, AddAssign, Display, From, Into};
use std::convert::Infallible;

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Add, AddAssign, From, Into, Display,
)]
#[display("{_0} superblock events")]
pub struct CheckpointEventCount(pub u64);

impl LayoutAs<u64> for CheckpointEventCount {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u64) -> Result<Self, Self::ReadError> {
        Ok(Self(v))
    }

    fn try_write(v: Self) -> Result<u64, Self::WriteError> {
        Ok(v.0)
    }
}
