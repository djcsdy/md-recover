use derive_more::{Deref, DerefMut, Display, From, Into};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display, Deref, DerefMut,
)]
#[display("inode #{_0}")]
pub struct InodeNumber(pub u32);
