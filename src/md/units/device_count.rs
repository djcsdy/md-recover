use binary_layout::LayoutAs;
use derive_more::{Add, AddAssign, Display, From, Into};
use std::convert::Infallible;

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Add, AddAssign, From, Into, Display,
)]
#[display("{_0} devices")]
pub struct DeviceCount(pub u32);

impl DeviceCount {
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self(self.0.checked_add(rhs.0)?))
    }
}

impl LayoutAs<u32> for DeviceCount {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.0)
    }
}

impl From<DeviceCount> for usize {
    fn from(value: DeviceCount) -> Self {
        usize::try_from(u32::from(value)).unwrap()
    }
}

impl From<DeviceCount> for u64 {
    fn from(value: DeviceCount) -> Self {
        u64::from(u32::from(value))
    }
}
