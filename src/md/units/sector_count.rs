use binary_layout::LayoutAs;
use derive_more::{Add, AddAssign, Display, From};
use std::convert::Infallible;
use std::ops::{Add, Sub};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Add, AddAssign, From, Display,
)]
pub struct SectorCount<C: PartialOrd + Add + Sub>(pub C);

impl From<SectorCount<u16>> for u16 {
    fn from(value: SectorCount<u16>) -> Self {
        value.0
    }
}

impl From<SectorCount<u32>> for u32 {
    fn from(value: SectorCount<u32>) -> Self {
        value.0
    }
}

impl From<SectorCount<u64>> for u64 {
    fn from(value: SectorCount<u64>) -> Self {
        value.0
    }
}

impl From<SectorCount<u16>> for SectorCount<u32> {
    fn from(value: SectorCount<u16>) -> Self {
        Self(value.0.into())
    }
}

impl From<SectorCount<u16>> for SectorCount<u64> {
    fn from(value: SectorCount<u16>) -> Self {
        Self(value.0.into())
    }
}

impl From<SectorCount<u32>> for SectorCount<u64> {
    fn from(value: SectorCount<u32>) -> Self {
        Self(value.0.into())
    }
}

impl LayoutAs<u32> for SectorCount<u32> {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.0)
    }
}

impl LayoutAs<u64> for SectorCount<u64> {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u64) -> Result<Self, Self::ReadError> {
        Ok(Self(v))
    }

    fn try_write(v: Self) -> Result<u64, Self::WriteError> {
        Ok(v.0)
    }
}
