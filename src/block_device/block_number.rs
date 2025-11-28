use derive_more::{Deref, DerefMut, Display, From, Into};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display, Deref, DerefMut,
)]
#[display("block #{_0}")]
pub struct BlockNumber(pub u64);
