use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Sub, SubAssign};

#[derive(
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Copy,
    Hash,
    Debug,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    From,
    Display,
    Deref,
    DerefMut,
)]
#[display("{_0} blocks")]
pub struct BlockCount(pub u64);
