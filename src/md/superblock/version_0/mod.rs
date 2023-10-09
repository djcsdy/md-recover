use std::ffi::OsStr;
use std::io::{Error, ErrorKind, Read, Result};

use crate::md::superblock::{ArrayUuid, Superblock};

mod little_endian;
mod big_endian;

pub enum SuperblockVersion0<S: AsRef<[u8]>> {
    LittleEndian(little_endian::View<S>),
    BigEndian(big_endian::View<S>),
}

impl SuperblockVersion0<[u8; little_endian::SIZE]> {
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        let mut buffer = [0u8; little_endian::SIZE];
        reader.read_exact(&mut buffer)?;
        if SuperblockVersion0::LittleEndian(little_endian::View::new(&buffer)).valid() {
            Ok(Self::LittleEndian(little_endian::View::new(buffer)))
        } else if SuperblockVersion0::BigEndian(big_endian::View::new(&buffer)).valid() {
            Ok(Self::BigEndian(big_endian::View::new(buffer)))
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}

impl<S: AsRef<[u8]>> SuperblockVersion0<S> {
    pub const SIZE: usize = if little_endian::SIZE == big_endian::SIZE {
        little_endian::SIZE
    } else {
        panic!()
    };

    fn magic(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.magic().read(),
            Self::BigEndian(view) => view.magic().read()
        }
    }

    fn valid_magic(&self) -> bool {
        self.magic() == 0xa92b4efc
    }

    fn valid_major_version(&self) -> bool {
        self.major_version() == 0
    }
}

impl<S: AsRef<[u8]>> SuperblockVersion0<S> {
    pub fn minor_version(&self) -> u32 {
        match self {
            SuperblockVersion0::LittleEndian(view) => view.minor_version().read(),
            SuperblockVersion0::BigEndian(view) => view.minor_version().read()
        }
    }

    fn array_uuid_0(&self) -> u32 {
        match self {
            SuperblockVersion0::LittleEndian(view) => view.array_uuid_0().read(),
            SuperblockVersion0::BigEndian(view) => view.array_uuid_0().read()
        }
    }

    fn array_uuid_all(&self) -> [u32; 4] {
        match self {
            SuperblockVersion0::LittleEndian(view) => [
                view.array_uuid_0().read(),
                view.array_uuid_1().read(),
                view.array_uuid_2().read(),
                view.array_uuid_3().read()
            ],
            SuperblockVersion0::BigEndian(view) => [
                view.array_uuid_0().read(),
                view.array_uuid_1().read(),
                view.array_uuid_2().read(),
                view.array_uuid_3().read()
            ]
        }
    }
}

impl<S: AsRef<[u8]>> Superblock for SuperblockVersion0<S> {
    fn valid(&self) -> bool {
        self.valid_magic() && self.valid_major_version()
    }

    fn major_version(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.major_version().read(),
            Self::BigEndian(view) => view.major_version().read()
        }
    }

    fn array_uuid(&self) -> ArrayUuid {
        if self.minor_version() < 90 {
            ArrayUuid::from_u32(self.array_uuid_0())
        } else {
            ArrayUuid::from_u32_4(&self.array_uuid_all())
        }
    }

    fn array_name(&self) -> Option<&OsStr> {
        None
    }
}
