use binary_layout::LayoutAs;
use derive_more::{Add, AddAssign, Display, From, Into};
use std::convert::Infallible;

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Add, AddAssign, From, Into, Display,
)]
#[display("{_0} devices")]
pub struct DeviceCount(pub u32);

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
