use crate::md::units::{DeviceCount, DeviceNumber, StripeNumber};
use derive_more::{Display, From, Into};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("chunk #{_0}")]
pub struct ChunkNumber(pub u64);

impl ChunkNumber {
    pub fn as_data_device_and_stripe_number(
        &self,
        data_device_count: DeviceCount,
    ) -> Option<(DeviceNumber, StripeNumber)> {
        Some((
            DeviceNumber(
                self.0
                    .checked_rem(u64::from(data_device_count))?
                    .try_into()
                    .ok()?,
            ),
            StripeNumber(self.0.checked_div(u64::from(data_device_count))?),
        ))
    }
}
