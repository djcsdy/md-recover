use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct Features: u32 {
        const BITMAP_OFFSET = 1;
        const RECOVERY_OFFSET = 2;
        const RESHAPE_ACTIVE = 4;
        const BAD_BLOCKS = 8;
        const REPLACEMENT = 16;
        const RESHAPE_BACKWARDS = 32;
        const NEW_OFFSET = 64;
        const BITMAP_VERSIONED = 256;
        const JOURNAL = 512;
        const PPL = 1024;
        const MULTIPLE_PPLS = 2048;
        const RAID0_LAYOUT = 4096;
    }
}

impl LayoutAs<u32> for Features {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.bits())
    }
}
