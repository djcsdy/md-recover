use std::fmt::{Display, Formatter};

use byteorder::{ByteOrder, LittleEndian};
use itertools::Itertools;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub enum ArrayUuid {
    Short([u8; 4]),
    Long([u8; 16]),
}

impl ArrayUuid {
    pub fn from_u32(value: u32) -> Self {
        let mut buffer = [0u8; 4];
        LittleEndian::write_u32(&mut buffer, value);
        Self::Short(buffer)
    }

    pub fn from_u32_4(value: &[u32; 4]) -> Self {
        let mut buffer = [0u8; 16];
        LittleEndian::write_u32_into(value.as_ref(), &mut buffer);
        Self::Long(buffer)
    }

    pub fn from_u8_16(value: &[u8; 16]) -> Self {
        Self::Long(*value)
    }
}

impl Display for ArrayUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayUuid::Short(uuid) => write!(f, "{:02x}", uuid.iter().format("")),
            ArrayUuid::Long(uuid) => write!(f, "{:02x}", uuid.iter().format("")),
        }
    }
}
