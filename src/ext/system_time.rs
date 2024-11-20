use std::time::{Duration, SystemTime};

pub trait SystemTimeExt {
    fn from_low_high(low: u32, high: u8) -> Self;
}

impl SystemTimeExt for SystemTime {
    fn from_low_high(low: u32, high: u8) -> Self {
        Self::UNIX_EPOCH + Duration::from_secs((low as u64) | ((high as u64) << 32))
    }
}
