use binary_layout::LayoutAs;
bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct State: u16 {
        const CLEANLY_UNMOUNTED = 1;
        const ERRORS_DETECTED = 2;
        const ORPHANS_BEING_RECOVERED = 4;
    }
}

impl LayoutAs<u16> for State {
    fn read(v: u16) -> Self {
        Self::from_bits_retain(v)
    }

    fn write(v: Self) -> u16 {
        v.bits()
    }
}
