use crate::ext4::units::BlockCount;
use binary_layout::LayoutAs;
use derive_more::{Deref, DerefMut, Display, From, Into};
use std::cmp::Ordering;
use std::convert::Infallible;

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display, Deref, DerefMut,
)]
#[display("file block #{_0}")]
pub struct FileBlockNumber(pub u32);

impl LayoutAs<u32> for FileBlockNumber {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.0)
    }
}

impl PartialEq<BlockCount<u32>> for FileBlockNumber {
    fn eq(&self, other: &BlockCount<u32>) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<BlockCount<u32>> for FileBlockNumber {
    fn partial_cmp(&self, other: &BlockCount<u32>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
