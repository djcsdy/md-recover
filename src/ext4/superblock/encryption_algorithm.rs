use binary_layout::LayoutAs;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::Infallible;

#[repr(u8)]
#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, IntoPrimitive, FromPrimitive,
)]
pub enum EncryptionAlgorithm {
    Invalid = 0,
    Aes256Xts = 1,
    Aes256Gcm = 2,
    Aes256Cbc = 3,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl LayoutAs<u8> for EncryptionAlgorithm {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u8) -> Result<Self, Self::ReadError> {
        Ok(v.into())
    }

    fn try_write(v: Self) -> Result<u8, Self::WriteError> {
        Ok(v.into())
    }
}
