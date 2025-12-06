use crate::md::units::{DeviceCount, DeviceNumber, SectorCount, SectorNumber};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum Raid6Algorithm {
    /// Rotating Parity N with Data Restart
    LeftAsymmetric,
    /// Rotating Parity 0 with Data Restart
    RightAsymmetric,
    /// Rotating Parity N with Data Continuation
    LeftSymmetric,
    /// Rotating Parity 0 with Data Continuation
    RightSymmetric,
    /// P and Q are initial devices
    Parity0,
    /// P and Q are final devices
    ParityN,
    /// DDF v1.2 PRL=6 RLQ=1
    Rotating0Restart,
    /// DDF v1.2 PRL=6 RLQ=2
    RotatingNRestart,
    /// DDF v1.2 PRL=6 RLQ=3
    RotatingNContinue,
    /// Same as LeftAsymmetric but with Q always on the last device
    LeftAsymmetric6,
    /// Same as RightAsymmetric but with Q always on the last device
    RightAsymmetric6,
    /// Same as LeftSymmetric but with Q always on the last device
    LeftSymmetric6,
    /// Same as RightSymmetric but with Q always on the last device
    RightSymmetric6,
    /// Same as Parity0 but with Q always on the last device
    Parity06,
}

impl Raid6Algorithm {
    pub fn from_layout(layout: u32) -> Option<Self> {
        match layout {
            0 => Some(Self::LeftAsymmetric),
            1 => Some(Self::RightAsymmetric),
            2 => Some(Self::LeftSymmetric),
            3 => Some(Self::RightSymmetric),
            4 => Some(Self::Parity0),
            5 => Some(Self::ParityN),
            8 => Some(Self::Rotating0Restart),
            9 => Some(Self::RotatingNRestart),
            10 => Some(Self::RotatingNContinue),
            16 => Some(Self::LeftAsymmetric6),
            17 => Some(Self::RightAsymmetric6),
            18 => Some(Self::LeftSymmetric6),
            19 => Some(Self::RightSymmetric6),
            20 => Some(Self::Parity06),
            _ => None,
        }
    }

    pub fn compute_sector(
        &self,
        sector_number: SectorNumber,
        sectors_per_chunk: SectorCount<u32>,
        raid_device_count: DeviceCount,
    ) -> Option<(SectorNumber, DeviceNumber, DeviceNumber, DeviceNumber)> {
        let (chunk_number, sector_in_chunk) = sector_number.in_chunk(sectors_per_chunk)?;
        let data_device_count = DeviceCount(u32::from(raid_device_count).checked_sub(2)?);
        let (data_device_number, stripe_number) =
            chunk_number.as_data_device_and_stripe_number(data_device_count)?;

        let p_device_number = DeviceNumber(
            match self {
                Raid6Algorithm::LeftAsymmetric
                | Raid6Algorithm::LeftSymmetric
                | Raid6Algorithm::RotatingNContinue => {
                    u64::from(raid_device_count).checked_sub(1)?.checked_sub(
                        u64::from(stripe_number).checked_rem(u64::from(raid_device_count))?,
                    )?
                }
                Raid6Algorithm::RightAsymmetric
                | Raid6Algorithm::RightSymmetric
                | Raid6Algorithm::Rotating0Restart => {
                    u64::from(stripe_number).checked_rem(u64::from(raid_device_count))?
                }
                Raid6Algorithm::Parity0 => 0,
                Raid6Algorithm::ParityN => u64::from(data_device_count),
                Raid6Algorithm::RotatingNRestart => {
                    u64::from(raid_device_count).checked_sub(1)?.checked_sub(
                        u64::from(stripe_number)
                            .checked_add(1)?
                            .checked_rem(u64::from(raid_device_count))?,
                    )?
                }
                Raid6Algorithm::LeftAsymmetric6 | Raid6Algorithm::LeftSymmetric6 => {
                    u64::from(data_device_count).checked_sub(
                        u64::from(stripe_number)
                            .checked_rem(u64::from(raid_device_count).checked_sub(1)?)?,
                    )?
                }
                Raid6Algorithm::RightAsymmetric6 | Raid6Algorithm::RightSymmetric6 => {
                    u64::from(stripe_number)
                        .checked_rem(u64::from(raid_device_count).checked_sub(1)?)?
                }
                Raid6Algorithm::Parity06 => 0,
            }
            .try_into()
            .ok()?,
        );

        let q_device_number = DeviceNumber(
            match self {
                Raid6Algorithm::LeftAsymmetric
                | Raid6Algorithm::RightAsymmetric
                | Raid6Algorithm::LeftSymmetric
                | Raid6Algorithm::RightSymmetric
                | Raid6Algorithm::Rotating0Restart
                | Raid6Algorithm::RotatingNRestart => u64::from(p_device_number)
                    .checked_add(1)?
                    .checked_rem(u64::from(raid_device_count))?,
                Raid6Algorithm::Parity0 => 1,
                Raid6Algorithm::ParityN => u64::from(data_device_count).checked_add(1)?,
                Raid6Algorithm::RotatingNContinue => u64::from(p_device_number)
                    .checked_add(u64::from(raid_device_count))?
                    .checked_sub(1)?
                    .checked_rem(u64::from(raid_device_count))?,
                Raid6Algorithm::LeftAsymmetric6
                | Raid6Algorithm::RightAsymmetric6
                | Raid6Algorithm::LeftSymmetric6
                | Raid6Algorithm::RightSymmetric6
                | Raid6Algorithm::Parity06 => u64::from(raid_device_count).checked_sub(1)?,
            }
            .try_into()
            .ok()?,
        );

        let data_device_number = DeviceNumber(
            match self {
                Raid6Algorithm::LeftAsymmetric
                | Raid6Algorithm::RightAsymmetric
                | Raid6Algorithm::Rotating0Restart
                | Raid6Algorithm::RotatingNRestart => u64::from(data_device_number).checked_add(
                    if q_device_number == DeviceNumber(0) {
                        1
                    } else if data_device_number >= p_device_number {
                        2
                    } else {
                        0
                    },
                )?,
                Raid6Algorithm::LeftSymmetric | Raid6Algorithm::RightSymmetric => {
                    u64::from(p_device_number)
                        .checked_add(2)?
                        .checked_add(u64::from(data_device_number))?
                        .checked_rem(u64::from(raid_device_count))?
                }
                Raid6Algorithm::Parity0 => u64::from(data_device_number).checked_add(2)?,
                Raid6Algorithm::ParityN => u64::from(data_device_number),
                Raid6Algorithm::RotatingNContinue => u64::from(p_device_number)
                    .checked_add(1)?
                    .checked_add(u64::from(data_device_number))?
                    .checked_rem(u64::from(raid_device_count))?,
                Raid6Algorithm::LeftAsymmetric6 | Raid6Algorithm::RightAsymmetric6 => u64::from(
                    data_device_number,
                )
                .checked_add(if data_device_number >= p_device_number {
                    1
                } else {
                    0
                })?,
                Raid6Algorithm::LeftSymmetric6 | Raid6Algorithm::RightSymmetric6 => {
                    u64::from(p_device_number)
                        .checked_add(1)?
                        .checked_add(u64::from(data_device_number))?
                        .checked_rem(u64::from(raid_device_count).checked_sub(1)?)?
                }
                Raid6Algorithm::Parity06 => u64::from(data_device_number).checked_add(1)?,
            }
            .try_into()
            .ok()?,
        );

        let sector_in_device = SectorNumber(
            u64::from(stripe_number)
                .checked_mul(u64::from(sectors_per_chunk))?
                .checked_add(u64::from(sector_in_chunk))?,
        );

        Some((
            sector_in_device,
            p_device_number,
            q_device_number,
            data_device_number,
        ))
    }
}
