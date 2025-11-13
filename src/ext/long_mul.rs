use std::ops::Mul;

pub trait LongMul: Sized {
    type Long: From<Self> + Mul<Output = Self::Long> + Sized;

    fn long_mul(self, rhs: Self) -> Self::Long {
        Self::Long::from(self).mul(rhs.into())
    }
}

impl LongMul for u8 {
    type Long = u16;
}

impl LongMul for u16 {
    type Long = u32;
}

impl LongMul for u32 {
    type Long = u64;
}

impl LongMul for u64 {
    type Long = u128;
}

impl LongMul for i8 {
    type Long = i16;
}

impl LongMul for i16 {
    type Long = i32;
}

impl LongMul for i32 {
    type Long = i64;
}

impl LongMul for i64 {
    type Long = i128;
}
