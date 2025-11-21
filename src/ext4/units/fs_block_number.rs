use derive_more::{Deref, DerefMut, Display, From, Into};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display, Deref, DerefMut,
)]
#[display("filesystem block #{_0}")]
pub struct FsBlockNumber(pub u64);
