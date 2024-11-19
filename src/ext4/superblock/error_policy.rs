use binary_layout::LayoutAs;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::Infallible;

#[repr(u16)]
#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, IntoPrimitive, FromPrimitive,
)]
pub enum ErrorPolicy {
    Continue = 1,
    RemountReadOnly = 2,
    Panic = 3,
    #[num_enum(catch_all)]
    Unknown(u16),
}

impl LayoutAs<u16> for ErrorPolicy {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u16) -> Result<Self, Self::ReadError> {
        Ok(v.into())
    }

    fn try_write(v: Self) -> Result<u16, Self::WriteError> {
        Ok(v.into())
    }
}
