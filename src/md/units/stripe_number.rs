use derive_more::{Display, From, Into};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into, Display)]
#[display("stripe #{_0}")]
pub struct StripeNumber(pub u64);
