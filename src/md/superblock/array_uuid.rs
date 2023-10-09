use std::fmt::{Display, Formatter};

use byteorder::{ByteOrder, LittleEndian};
use itertools::Itertools;

pub enum ArrayUuid {
    Short([u8; 4]),
    Long(LongArrayUuid)
}

pub struct LongArrayUuid([u8; 16]);

impl ArrayUuid {
    pub fn from_u32(value: u32) -> Self {
        let mut buffer = [0u8; 4];
        LittleEndian::write_u32(&mut buffer, value);
        Self::Short(buffer)
    }

    pub fn from_u32_4(value: &[u32; 4]) -> Self {
        let mut buffer = [0u8; 16];
        LittleEndian::write_u32_into(value.as_ref(), &mut buffer);
        Self::Long(LongArrayUuid::new(&buffer))
    }

    pub fn from_u8_16(value: &[u8; 16]) -> Self {
        Self::Long(LongArrayUuid::new(value))
    }
}

impl LongArrayUuid {
    pub fn new(value: &[u8; 16]) -> Self {
        Self(*value)
    }
}

impl Display for ArrayUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayUuid::Short(uuid) => write!(f, "{:02x}", uuid.iter().format("")),
            ArrayUuid::Long(uuid) => write!(f, "{}", uuid)
        }
    }
}

impl Display for LongArrayUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}", self.0.iter().format(""))
    }
}