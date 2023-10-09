use crate::md::superblock::Superblock;

mod little_endian;
mod big_endian;

pub enum SuperblockVersion0<S: AsRef<[u8]>> {
    LittleEndian(little_endian::View<S>),
    BigEndian(big_endian::View<S>),
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
}