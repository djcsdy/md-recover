use crate::md::raid5::Raid5Algorithm;
use crate::md::raid6::Raid6Algorithm;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum MdAlgorithm {
    Unsupported { level: u32, layout: u32 },
    Raid5(Raid5Algorithm),
    Raid6(Raid6Algorithm),
}

impl MdAlgorithm {
    pub fn from_level_and_layout(level: u32, layout: u32) -> Self {
        match level {
            5 => Raid5Algorithm::from_layout(layout).map(Self::Raid5),
            6 => Raid6Algorithm::from_layout(layout).map(Self::Raid6),
            _ => None,
        }
        .unwrap_or(Self::Unsupported { level, layout })
    }
}
