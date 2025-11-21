use crate::ext4::units::BlockCount;
use derive_more::{Deref, DerefMut, Display, From, Into};
use std::cmp::Ordering;

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display, Deref, DerefMut,
)]
#[display("filesystem block #{_0}")]
pub struct FsBlockNumber(pub u64);

impl PartialEq<BlockCount<u64>> for FsBlockNumber {
    fn eq(&self, other: &BlockCount<u64>) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<BlockCount<u64>> for FsBlockNumber {
    fn partial_cmp(&self, other: &BlockCount<u64>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
