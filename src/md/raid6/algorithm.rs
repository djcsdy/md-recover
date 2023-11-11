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
        sector: u64,
        sectors_per_chunk: u32,
        raid_disks: u32,
    ) -> (u64, u32, u32, u32) {
        let sector_in_chunk = sector % sectors_per_chunk as u64;
        let chunk_index = sector / sectors_per_chunk as u64;
        let data_disks = raid_disks - 2;
        let data_disk_index = (chunk_index % data_disks as u64) as u32;
        let stripe = (chunk_index / data_disks as u64) as u32;

        let p_disk = match self {
            Raid6Algorithm::LeftAsymmetric
            | Raid6Algorithm::LeftSymmetric
            | Raid6Algorithm::RotatingNContinue => raid_disks - 1 - stripe % raid_disks,
            Raid6Algorithm::RightAsymmetric
            | Raid6Algorithm::RightSymmetric
            | Raid6Algorithm::Rotating0Restart => stripe % raid_disks,
            Raid6Algorithm::Parity0 => 0,
            Raid6Algorithm::ParityN => data_disks,
            Raid6Algorithm::RotatingNRestart => raid_disks - 1 - (stripe + 1) % raid_disks,
            Raid6Algorithm::LeftAsymmetric6 | Raid6Algorithm::LeftSymmetric6 => {
                data_disks - stripe % (raid_disks - 1)
            }
            Raid6Algorithm::RightAsymmetric6 | Raid6Algorithm::RightSymmetric6 => {
                stripe % (raid_disks - 1)
            }
            Raid6Algorithm::Parity06 => 0,
        };

        let q_disk = match self {
            Raid6Algorithm::LeftAsymmetric
            | Raid6Algorithm::RightAsymmetric
            | Raid6Algorithm::LeftSymmetric
            | Raid6Algorithm::RightSymmetric
            | Raid6Algorithm::Rotating0Restart
            | Raid6Algorithm::RotatingNRestart => (p_disk + 1) % raid_disks,
            Raid6Algorithm::Parity0 => 1,
            Raid6Algorithm::ParityN => data_disks + 1,
            Raid6Algorithm::RotatingNContinue => (p_disk + raid_disks - 1) % raid_disks,
            Raid6Algorithm::LeftAsymmetric6
            | Raid6Algorithm::RightAsymmetric6
            | Raid6Algorithm::LeftSymmetric6
            | Raid6Algorithm::RightSymmetric6
            | Raid6Algorithm::Parity06 => raid_disks - 1,
        };

        let data_disk = match self {
            Raid6Algorithm::LeftAsymmetric
            | Raid6Algorithm::RightAsymmetric
            | Raid6Algorithm::Rotating0Restart
            | Raid6Algorithm::RotatingNRestart => {
                data_disk_index
                    + if q_disk == 0 {
                        1
                    } else if data_disk_index >= p_disk {
                        2
                    } else {
                        0
                    }
            }
            Raid6Algorithm::LeftSymmetric | Raid6Algorithm::RightSymmetric => {
                (p_disk + 2 + data_disk_index) % raid_disks
            }
            Raid6Algorithm::Parity0 => data_disk_index + 2,
            Raid6Algorithm::ParityN => data_disk_index,
            Raid6Algorithm::RotatingNContinue => (p_disk + 1 + data_disk_index) % raid_disks,
            Raid6Algorithm::LeftAsymmetric6 | Raid6Algorithm::RightAsymmetric6 => {
                data_disk_index + if data_disk_index >= p_disk { 1 } else { 0 }
            }
            Raid6Algorithm::LeftSymmetric6 | Raid6Algorithm::RightSymmetric6 => {
                (p_disk + 1 + data_disk_index) % (raid_disks - 1)
            }
            Raid6Algorithm::Parity06 => data_disk_index + 1,
        };

        let new_sector = chunk_index * sectors_per_chunk as u64 + sector_in_chunk;

        (new_sector, p_disk, q_disk, data_disk)
    }
}
