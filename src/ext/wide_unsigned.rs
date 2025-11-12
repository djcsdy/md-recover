use std::ops::{BitOr, Shl};

pub trait WideUnsigned: Sized + BitOr<Output = Self> + Shl<usize, Output = Self> {
    type Half: Into<Self> + Sized;

    fn from_low_high(low: Self::Half, high: Self::Half) -> Self {
        Into::<Self>::into(low) | (Into::<Self>::into(high) << (size_of::<Self::Half>() * 8))
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

#[test]
fn test_u16() {
    assert_eq!(u16::from_low_high(0xab, 0xcd), 0xcdab);
}

#[test]
fn test_u32() {
    assert_eq!(u32::from_low_high(0x1234, 0x5678), 0x56781234);
}

#[test]
fn test_u64() {
    assert_eq!(
        u64::from_low_high(0x12345678, 0x9abcdef0),
        0x9abcdef012345678
    );
}

#[test]
fn test_u128() {
    assert_eq!(
        u128::from_low_high(0x123456789abcdef0, 0x0fedcba987654321),
        0x0fedcba987654321123456789abcdef0
    );
}
