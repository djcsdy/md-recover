use binary_layout::LayoutAs;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::Infallible;

#[repr(u32)]
#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, IntoPrimitive, FromPrimitive,
)]
pub enum CreatorOs {
    Linux = 0,
    Hurd = 1,
    Masix = 2,
    FreeBsd = 3,
    Lites = 4,
    #[num_enum(catch_all)]
    Unknown(u32),
}

impl LayoutAs<u32> for CreatorOs {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(v.into())
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.into())
    }
}
