use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
    pub struct Flags : u16 {
        const INODE_NOT_IN_USE = 1;
        const BLOCK_BITMAP_NOT_IN_USE = 2;
        const INODE_TABLE_ZEROED = 4;
    }
}

impl LayoutAs<u16> for Flags {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u16) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u16, Self::WriteError> {
        Ok(v.bits())
    }
}
