use crate::md::units::{ChunkNumber, SectorCount};
use derive_more::{Display, From, Into};
use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("sector #{_0}")]
pub struct SectorNumber(pub u64);

impl SectorNumber {
    pub fn in_chunk<C>(
        &self,
        sectors_per_chunk: SectorCount<C>,
    ) -> Option<(ChunkNumber, SectorNumber)>
    where
        C: PartialOrd + Add + Sub + Copy + Into<u64>,
    {
        Some((
            ChunkNumber(self.0.checked_div(sectors_per_chunk.0.into())?),
            SectorNumber(self.0.checked_rem(sectors_per_chunk.0.into())?),
        ))
    }
}

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
