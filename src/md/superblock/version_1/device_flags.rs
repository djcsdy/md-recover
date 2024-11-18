use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct DeviceFlags: u8 {
        const WRITE_MOSTLY = 1;
        const FAIL_FAST = 2;
    }
}

impl LayoutAs<u8> for DeviceFlags {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u8) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u8, Self::WriteError> {
        Ok(v.bits())
    }
}
