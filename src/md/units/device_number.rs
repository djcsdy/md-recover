use crate::md::units::DeviceCount;
use derive_more::{Display, From, Into};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("md device #{_0}")]
pub struct DeviceNumber(pub u32);

impl PartialEq<DeviceCount> for DeviceNumber {
    fn eq(&self, other: &DeviceCount) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<DeviceCount> for DeviceNumber {
    fn partial_cmp(&self, other: &DeviceCount) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<DeviceNumber> for u64 {
    fn from(value: DeviceNumber) -> Self {
        value.0.into()
    }
}
