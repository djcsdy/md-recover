use derive_more::{Display, From, Into};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("sector #{_0}")]
pub struct SectorNumber(pub u64);
