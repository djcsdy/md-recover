use binary_layout::LayoutAs;
use derive_more::{Deref, DerefMut, Display, From, Into};
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
