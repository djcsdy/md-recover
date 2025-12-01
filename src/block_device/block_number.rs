use crate::block_device::{BlockCount, BlockSize};
use derive_more::{Display, From, Into};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("block #{_0}")]
pub struct BlockNumber(pub u64);

impl BlockNumber {
    pub(in crate::block_device) fn byte_pos(self, block_size: BlockSize) -> Option<u64> {
        u64::from(self).checked_mul(u64::from(block_size))
    }
}

impl Add<BlockCount> for BlockNumber {
    type Output = Self;

    fn add(self, rhs: BlockCount) -> Self::Output {
        Self(self.0 + u64::from(rhs))
    }
}

impl AddAssign<BlockCount> for BlockNumber {
    fn add_assign(&mut self, rhs: BlockCount) {
        self.0 += u64::from(rhs)
    }
}

impl PartialEq<BlockCount> for BlockNumber {
    fn eq(&self, other: &BlockCount) -> bool {
        self.0 == u64::from(*other)
    }
}

impl PartialOrd<BlockCount> for BlockNumber {
    fn partial_cmp(&self, other: &BlockCount) -> Option<Ordering> {
        self.0.partial_cmp(&u64::from(*other))
    }
}
