use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;
use crate::md::superblock::version_0::device_descriptor::DeviceDescriptor;
use crate::md::superblock::version_0::{big_endian, little_endian};
use crate::md::superblock::{ArrayUuid, Superblock};
use std::ffi::OsStr;
use std::io;
use std::io::{Error, ErrorKind, Read};

pub enum SuperblockVersion0<S: AsRef<[u8]>> {
    LittleEndian(little_endian::View<S>),
    BigEndian(big_endian::View<S>),
}

impl SuperblockVersion0<[u8; little_endian::SIZE]> {
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut buffer = [0u8; little_endian::SIZE];
        reader.read_exact(&mut buffer)?;
        if SuperblockVersion0::LittleEndian(little_endian::View::new(&buffer)).valid_magic() {
            if SuperblockVersion0::LittleEndian(little_endian::View::new(&buffer)).valid() {
                Ok(Self::LittleEndian(little_endian::View::new(buffer)))
            } else {
                Err(Error::from(ErrorKind::InvalidData))
            }
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

    pub(super) const MAX_DEVICES: usize = 27;

    fn magic(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.magic().read(),
            Self::BigEndian(view) => view.magic().read(),
        }
    }

    fn valid_magic(&self) -> bool {
        self.magic() == 0xa92b4efc
    }

    fn valid_major_version(&self) -> bool {
        self.major_version() == 0
    }

    pub fn minor_version(&self) -> u32 {
        match self {
            SuperblockVersion0::LittleEndian(view) => view.minor_version().read(),
            SuperblockVersion0::BigEndian(view) => view.minor_version().read(),
        }
    }

    fn array_uuid_0(&self) -> u32 {
        match self {
            SuperblockVersion0::LittleEndian(view) => view.array_uuid_0().read(),
            SuperblockVersion0::BigEndian(view) => view.array_uuid_0().read(),
        }
    }

    fn array_uuid_all(&self) -> [u32; 4] {
        match self {
            SuperblockVersion0::LittleEndian(view) => [
                view.array_uuid_0().read(),
                view.array_uuid_1().read(),
                view.array_uuid_2().read(),
                view.array_uuid_3().read(),
            ],
            SuperblockVersion0::BigEndian(view) => [
                view.array_uuid_0().read(),
                view.array_uuid_1().read(),
                view.array_uuid_2().read(),
                view.array_uuid_3().read(),
            ],
        }
    }

    fn level(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.level().read(),
            Self::BigEndian(view) => view.level().read(),
        }
    }

    fn layout(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.layout().read(),
            Self::BigEndian(view) => view.layout().read(),
        }
    }

    fn valid_device_descriptors(&self) -> bool {
        match self {
            SuperblockVersion0::LittleEndian(view) => {
                let buffer = view.disks();
                (0..Self::MAX_DEVICES)
                    .map(|i| {
                        little_endian::DeviceDescriptorLittleEndian::new(array_ref![
                            buffer,
                            i * little_endian::DeviceDescriptorLittleEndian::<&[u8]>::SIZE,
                            little_endian::DeviceDescriptorLittleEndian::<&[u8]>::SIZE
                        ])
                    })
                    .all(|descriptor| descriptor.is_valid())
            }
            SuperblockVersion0::BigEndian(view) => {
                let buffer = view.disks();
                (0..Self::MAX_DEVICES)
                    .map(|i| {
                        big_endian::DeviceDescriptorBigEndian::new(array_ref![
                            buffer,
                            i * little_endian::DeviceDescriptorLittleEndian::<&[u8]>::SIZE,
                            little_endian::DeviceDescriptorLittleEndian::<&[u8]>::SIZE
                        ])
                    })
                    .all(|descriptor| descriptor.is_valid())
            }
        }
    }
}

impl<S: AsRef<[u8]>> Superblock for SuperblockVersion0<S> {
    fn valid(&self) -> bool {
        self.valid_magic() && self.valid_major_version() && self.valid_device_descriptors()
    }

    fn major_version(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.major_version().read(),
            Self::BigEndian(view) => view.major_version().read(),
        }
    }

    fn minor_version(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.minor_version().read(),
            Self::BigEndian(view) => view.minor_version().read(),
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

    fn algorithm(&self) -> MdAlgorithm {
        MdAlgorithm::from_level_and_layout(self.level(), self.layout())
    }

    fn size(&self) -> u64 {
        match self {
            Self::LittleEndian(view) => view.size().read().into(),
            Self::BigEndian(view) => view.size().read().into(),
        }
    }

    fn chunk_size(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.chunk_size().read(),
            Self::BigEndian(view) => view.chunk_size().read(),
        }
    }

    fn raid_disks(&self) -> u32 {
        match self {
            Self::LittleEndian(view) => view.raid_disks().read(),
            Self::BigEndian(view) => view.raid_disks().read(),
        }
    }

    fn reshape_status(&self) -> ReshapeStatus {
        match self {
            Self::LittleEndian(view) => view.reshape_status().into(),
            Self::BigEndian(view) => view.reshape_status().into(),
        }
    }

    fn event_count(&self) -> u64 {
        match self {
            SuperblockVersion0::LittleEndian(view) => view.event_count().read(),
            SuperblockVersion0::BigEndian(view) => view.event_count().read(),
        }
    }

    fn device_roles(&self) -> Vec<u16> {
        match self {
            SuperblockVersion0::LittleEndian(view) => {
                let buffer = view.disks();
                Vec::from_iter((0..Self::MAX_DEVICES).map(|i| {
                    little_endian::DeviceDescriptorLittleEndian::new(array_ref![
                        buffer,
                        i * little_endian::DeviceDescriptorLittleEndian::<&[u8]>::SIZE,
                        little_endian::DeviceDescriptorLittleEndian::<&[u8]>::SIZE
                    ])
                    .role()
                    .try_into()
                    .unwrap()
                }))
            }
            SuperblockVersion0::BigEndian(view) => {
                let buffer = view.disks();
                Vec::from_iter((0..Self::MAX_DEVICES).map(|i| {
                    big_endian::DeviceDescriptorBigEndian::new(array_ref![
                        buffer,
                        i * big_endian::DeviceDescriptorBigEndian::<&[u8]>::SIZE,
                        big_endian::DeviceDescriptorBigEndian::<&[u8]>::SIZE
                    ])
                    .role()
                    .try_into()
                    .unwrap()
                }))
            }
        }
    }
}
