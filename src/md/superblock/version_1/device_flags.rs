use binary_layout::LayoutAs;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct DeviceFlags: u8 {
        const WRITE_MOSTLY = 1;
        const FAIL_FAST = 2;
    }
}

impl LayoutAs<u8> for DeviceFlags {
    fn read(v: u8) -> Self {
        Self::from_bits_retain(v)
    }

    fn write(v: Self) -> u8 {
        v.bits()
    }
}
