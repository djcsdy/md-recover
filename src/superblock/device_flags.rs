bitflags! {
    pub struct DeviceFlags: u8 {
        const WRITE_MOSTLY = 1;
        const FAIL_FAST = 2;
    }
}
