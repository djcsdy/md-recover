use binary_layout::LayoutAs;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::Infallible;

#[repr(u8)]
#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, IntoPrimitive, FromPrimitive,
)]
pub enum FileType {
    Socket = 12,
    SymbolicLink = 10,
    RegularFile = 8,
    BlockDevice = 6,
    Directory = 4,
    CharacterDevice = 2,
    Fifo = 1,
    #[num_enum(catch_all)]
    Unknown(u8) = 0,
}

impl LayoutAs<u8> for FileType {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u8) -> Result<Self, Self::ReadError> {
        Ok(v.into())
    }

    fn try_write(v: Self) -> Result<u8, Self::WriteError> {
        Ok(v.into())
    }
}
