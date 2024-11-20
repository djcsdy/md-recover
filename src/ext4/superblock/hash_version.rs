use binary_layout::LayoutAs;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::Infallible;

#[repr(u8)]
#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, IntoPrimitive, FromPrimitive,
)]
pub enum HashVersion {
    Legacy = 0,
    HalfMd4 = 1,
    Tea = 2,
    LegacyUnsigned = 3,
    HalfMd4Unsigned = 4,
    TeaUnsigned = 5,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl LayoutAs<u8> for HashVersion {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u8) -> Result<Self, Self::ReadError> {
        Ok(v.into())
    }

    fn try_write(v: Self) -> Result<u8, Self::WriteError> {
        Ok(v.into())
    }
}
