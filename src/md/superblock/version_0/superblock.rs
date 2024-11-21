use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;
use crate::md::superblock::version_0::{big_endian, little_endian};
use crate::md::superblock::{ArrayUuid, Superblock};
use std::ffi::OsStr;
use std::io;
use std::io::{Error, ErrorKind, Read};

const SIZE: usize = if little_endian::SIZE == big_endian::SIZE {
    little_endian::SIZE
} else {
    panic!()
};

pub(super) const MAX_DEVICES: usize = 27;

pub fn read_superblock_version_0<R: Read>(mut reader: R) -> io::Result<Box<dyn Superblock>> {
    let mut buffer = [0u8; SIZE];
    reader.read_exact(&mut buffer)?;
    let little_endian = little_endian::View::new(buffer);
    if little_endian.valid_magic() {
        if little_endian.valid() {
            Ok(Box::new(little_endian))
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    } else {
        let big_endian = big_endian::View::new(little_endian.into_storage());
        if big_endian.valid() {
            Ok(Box::new(big_endian))
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}

pub trait SuperblockVersion0 {
    fn valid_magic(&self) -> bool {
        self.magic() == 0xa92b4efc
    }

    fn valid_major_version(&self) -> bool {
        self.major_version() == 0
    }

    fn valid_device_descriptors(&self) -> bool;
    fn magic(&self) -> u32;
    fn major_version(&self) -> u32;
    fn minor_version(&self) -> u32;
    fn array_uuid_0(&self) -> u32;
    fn level(&self) -> u32;
    fn size(&self) -> u32;
    fn raid_disks(&self) -> u32;
    fn array_uuid_1(&self) -> u32;
    fn array_uuid_2(&self) -> u32;
    fn array_uuid_3(&self) -> u32;
    fn event_count(&self) -> u64;
    fn reshape_status(&self) -> ReshapeStatus;
    fn layout(&self) -> u32;
    fn chunk_size(&self) -> u32;
    fn device_roles(&self) -> Vec<u16>;
}

impl<S: SuperblockVersion0> Superblock for S {
    fn valid(&self) -> bool {
        self.valid_magic() && self.valid_major_version() && self.valid_device_descriptors()
    }

    fn major_version(&self) -> u32 {
        self.major_version()
    }

    fn minor_version(&self) -> u32 {
        self.minor_version()
    }

    fn array_uuid(&self) -> ArrayUuid {
        if self.minor_version() < 90 {
            ArrayUuid::from_u32(self.array_uuid_0())
        } else {
            ArrayUuid::from_u32_4(&[
                self.array_uuid_0(),
                self.array_uuid_1(),
                self.array_uuid_2(),
                self.array_uuid_3(),
            ])
        }
    }

    fn array_name(&self) -> Option<&OsStr> {
        None
    }

    fn algorithm(&self) -> MdAlgorithm {
        MdAlgorithm::from_level_and_layout(self.level(), self.layout())
    }

    fn size(&self) -> u64 {
        self.size().into()
    }

    fn chunk_size(&self) -> u32 {
        self.chunk_size()
    }

    fn raid_disks(&self) -> u32 {
        self.raid_disks()
    }

    fn reshape_status(&self) -> ReshapeStatus {
        self.reshape_status()
    }

    fn event_count(&self) -> u64 {
        self.event_count()
    }

    fn device_roles(&self) -> Vec<u16> {
        self.device_roles()
    }
}
