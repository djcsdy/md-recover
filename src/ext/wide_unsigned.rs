use std::ops::{BitOr, Shl};

pub trait WideUnsigned: Sized + BitOr<Output = Self> + Shl<usize, Output = Self> {
    type Half: Into<Self> + Sized;

    fn from_low_high(low: Self::Half, high: Self::Half) -> Self {
        Into::<Self>::into(low) | (Into::<Self>::into(high) << size_of::<Self::Half>())
    }
}

impl WideUnsigned for u16 {
    type Half = u8;
}

impl WideUnsigned for u32 {
    type Half = u16;
}

impl WideUnsigned for u64 {
    type Half = u32;
}

impl WideUnsigned for u128 {
    type Half = u64;
}
