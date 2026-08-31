use crate::lvm::SECTOR_SIZE_BYTES;
use derive_more::{Display, From, Into};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("sector #{_0}")]
pub struct SectorNumber(pub u64);

impl SectorNumber {
    pub(in crate::lvm) fn byte_pos(&self) -> Option<u64> {
        self.0
            .checked_mul(u64::try_from(SECTOR_SIZE_BYTES).unwrap())
    }
}
