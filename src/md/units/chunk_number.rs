use derive_more::{Display, From, Into};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("chunk #{_0}")]
pub struct ChunkNumber(pub u64);
