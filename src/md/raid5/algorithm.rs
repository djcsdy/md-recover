use crate::md::units::{DeviceCount, DeviceNumber, SectorCount, SectorNumber};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum Raid5Algorithm {
    /// Rotating Parity N with Data Restart
    LeftAsymmetric,
    /// Rotating Parity 0 with Data Restart
    RightAsymmetric,
    /// Rotating Parity N with Data Continuation
    LeftSymmetric,
    /// Rotating Parity 0 with Data Continuation
    RightSymmetric,
    /// P is initial device
    Parity0,
    /// P is final device
    ParityN,
}

impl Raid5Algorithm {
    pub fn from_layout(layout: u32) -> Option<Self> {
        match layout {
            0 => Some(Self::LeftAsymmetric),
            1 => Some(Self::RightAsymmetric),
            2 => Some(Self::LeftSymmetric),
            3 => Some(Self::RightSymmetric),
            4 => Some(Self::Parity0),
            5 => Some(Self::ParityN),
            _ => None,
        }
    }

    pub fn compute_sector(
        &self,
        sector_number: SectorNumber,
        sectors_per_chunk: SectorCount<u32>,
        raid_device_count: DeviceCount,
    ) -> Option<(SectorNumber, DeviceNumber, DeviceNumber)> {
        let (chunk_number, sector_in_chunk) = sector_number.in_chunk(sectors_per_chunk)?;
        let data_device_count = DeviceCount(u32::from(raid_device_count).checked_sub(1)?);
        let (data_device_number, stripe_number) =
            chunk_number.as_data_device_and_stripe_number(data_device_count)?;

        let parity_device_number = DeviceNumber(
            match self {
                Raid5Algorithm::LeftAsymmetric | Raid5Algorithm::LeftSymmetric => u64::from(
                    data_device_count,
                )
                .checked_sub(u64::from(stripe_number).checked_rem(u64::from(raid_device_count))?)?,
                Raid5Algorithm::RightAsymmetric | Raid5Algorithm::RightSymmetric => {
                    u64::from(stripe_number).checked_rem(u64::from(raid_device_count))?
                }
                Raid5Algorithm::Parity0 => 0,
                Raid5Algorithm::ParityN => u64::from(data_device_count),
            }
            .try_into()
            .ok()?,
        );

        let data_device_number = DeviceNumber(
            match self {
                Raid5Algorithm::LeftAsymmetric | Raid5Algorithm::RightAsymmetric => u64::from(
                    data_device_number,
                )
                .checked_add(if data_device_number >= parity_device_number {
                    1
                } else {
                    0
                })?,
                Raid5Algorithm::LeftSymmetric | Raid5Algorithm::RightSymmetric => {
                    u64::from(parity_device_number)
                        .checked_add(1)?
                        .checked_add(u64::from(data_device_number))?
                        .checked_rem(u64::from(raid_device_count))?
                }
                Raid5Algorithm::Parity0 => u64::from(data_device_number).checked_add(1)?,
                Raid5Algorithm::ParityN => u64::from(data_device_number),
            }
            .try_into()
            .ok()?,
        );

        let sector_in_device = SectorNumber(
            u64::from(chunk_number)
                .checked_mul(u64::from(sectors_per_chunk))?
                .checked_add(u64::from(sector_in_chunk))?,
        );

        Some((sector_in_device, parity_device_number, data_device_number))
    }
}
