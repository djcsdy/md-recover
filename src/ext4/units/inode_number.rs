use crate::ext4::units::inode_count::InodeCount;
use derive_more::{Deref, DerefMut, Display, From, Into};
use std::cmp::Ordering;

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display, Deref, DerefMut,
)]
#[display("inode #{_0}")]
pub struct InodeNumber(pub u32);

impl PartialEq<InodeCount> for InodeNumber {
    fn eq(&self, other: &InodeCount) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<InodeCount> for InodeNumber {
    fn partial_cmp(&self, other: &InodeCount) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
