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
        sector: u64,
        sectors_per_chunk: u32,
        raid_disks: u32,
    ) -> (u64, u32, u32) {
        let sector_in_chunk = sector % sectors_per_chunk as u64;
        let chunk_index = sector / sectors_per_chunk as u64;
        let data_disks = raid_disks - 1;
        let data_disk_index = (chunk_index % data_disks as u64) as u32;
        let stripe = (chunk_index / data_disks as u64) as u32;

        let parity_disk = match self {
            Raid5Algorithm::LeftAsymmetric | Raid5Algorithm::LeftSymmetric => {
                data_disks - stripe % raid_disks
            }
            Raid5Algorithm::RightAsymmetric | Raid5Algorithm::RightSymmetric => stripe % raid_disks,
            Raid5Algorithm::Parity0 => 0,
            Raid5Algorithm::ParityN => data_disks,
        };

        let data_disk = match self {
            Raid5Algorithm::LeftAsymmetric | Raid5Algorithm::RightAsymmetric => {
                data_disk_index + if data_disk_index >= parity_disk { 1 } else { 0 }
            }
            Raid5Algorithm::LeftSymmetric | Raid5Algorithm::RightSymmetric => {
                (parity_disk + 1 + data_disk_index) % raid_disks
            }
            Raid5Algorithm::Parity0 => data_disk_index + 1,
            Raid5Algorithm::ParityN => data_disk_index,
        };

        let new_sector = chunk_index * sectors_per_chunk as u64 + sector_in_chunk;

        (new_sector, parity_disk, data_disk)
    }
}
