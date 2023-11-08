use crate::md::raid5::Raid5Algorithm;
use crate::md::raid6::Raid6Algorithm;

pub enum MdAlgorithm {
    Unsupported {
        level: u32,
        layout: u32,
    },
    Raid5(Raid5Algorithm),
    Raid6(Raid6Algorithm),
}