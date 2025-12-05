use crate::md::units::SectorCount;
use derive_more::{Display, From, Into};
use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("sector #{_0}")]
pub struct SectorNumber(pub u64);

impl<C> PartialEq<SectorCount<C>> for SectorNumber
where
    C: PartialOrd + Add + Sub + Copy + Into<u64>,
{
    fn eq(&self, other: &SectorCount<C>) -> bool {
        self.0 == other.0.into()
    }
}

impl<C> PartialOrd<SectorCount<C>> for SectorNumber
where
    C: PartialOrd + Add + Sub + Copy + Into<u64>,
{
    fn partial_cmp(&self, other: &SectorCount<C>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0.into())
    }
}
