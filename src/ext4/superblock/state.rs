use binary_layout::LayoutAs;
use std::convert::Infallible;
bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct State: u16 {
        const CLEANLY_UNMOUNTED = 1;
        const ERRORS_DETECTED = 2;
        const ORPHANS_BEING_RECOVERED = 4;
    }
}

impl LayoutAs<u16> for State {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u16) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u16, Self::WriteError> {
        Ok(v.bits())
    }
}
