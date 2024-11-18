use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct Flags: u32 {
        /// If set, signed directory hash is in use.
        const SIGNED_DIRECTORY_HASH = 1;

        /// If set, unsigned directory hash is in use.
        const UNSIGNED_DIRECTORY_HASH = 2;

        /// If set, this is a test filesystem ok for use with experimental code.
        const TEST_FILESYSTEM = 4;
    }
}

impl LayoutAs<u32> for Flags {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.bits())
    }
}
