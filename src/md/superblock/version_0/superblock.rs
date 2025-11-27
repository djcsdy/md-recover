use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;
use crate::md::superblock::version_0::device_descriptor::DeviceDescriptor;
use crate::md::superblock::version_0::{big_endian, little_endian};
use crate::md::superblock::{ArrayUuid, MdDeviceRole, Superblock};
use std::ffi::OsStr;
use std::io;
use std::io::Read;

pub struct SuperblockVersion0 {
    pub(super) magic: u32,
    pub(super) major_version: u32,
    pub(super) minor_version: u32,
    pub(super) patch_version: u32,
    pub(super) gvalid_words: u32,
    pub(super) array_uuid_0: u32,
    pub(super) ctime: u32,
    pub(super) level: u32,
    pub(super) size: u32,
    pub(super) nr_disks: u32,
    pub(super) raid_disks: u32,
    pub(super) md_minor: u32,
    pub(super) not_persistent: u32,
    pub(super) array_uuid_1: u32,
    pub(super) array_uuid_2: u32,
    pub(super) array_uuid_3: u32,
    pub(super) utime: u32,
    pub(super) state: u32,
    pub(super) active_disks: u32,
    pub(super) working_disks: u32,
    pub(super) failed_disks: u32,
    pub(super) spare_disks: u32,
    pub(super) superblock_checksum: u32,
    pub(super) event_count: u64,
    pub(super) checkpoint_event_count: u64,
    pub(super) recovery_checkpoint: u32,
    pub(super) reshape_status: ReshapeStatus,
    pub(super) layout: u32,
    pub(super) chunk_size: u32,
    pub(super) root_pv: u32,
    pub(super) root_block: u32,
    pub(super) disks: Vec<DeviceDescriptor>,
}

impl SuperblockVersion0 {
    pub const SIZE_ON_DISK: usize = if little_endian::SIZE == big_endian::SIZE {
        little_endian::SIZE
    } else {
        panic!()
    };

    pub const MAX_DEVICES: usize = 27;

    pub const MAGIC: u32 = 0xa92b4efc;

    pub const MAJOR_VERSION: u32 = 0;

    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut buffer = [0u8; Self::SIZE_ON_DISK];
        reader.read_exact(&mut buffer)?;
        let little_endian = little_endian::View::new(buffer);
        let superblock: SuperblockVersion0 = if little_endian.magic().read() == Self::MAGIC {
            little_endian.into()
        } else {
            big_endian::View::new(little_endian.into_storage()).into()
        };
        if superblock.valid() {
            Ok(superblock)
        } else {
            Err(io::ErrorKind::InvalidData.into())
        }
    }

    fn valid_magic(&self) -> bool {
        self.magic == Self::MAGIC
    }

    fn valid_major_version(&self) -> bool {
        self.major_version == Self::MAJOR_VERSION
    }

    fn valid_device_descriptors(&self) -> bool {
        self.disks
            .iter()
            .take(self.nr_disks as usize)
            .all(|descriptor| descriptor.is_valid())
    }
}

impl Superblock for SuperblockVersion0 {
    fn valid(&self) -> bool {
        self.valid_magic() && self.valid_major_version() && self.valid_device_descriptors()
    }

    fn major_version(&self) -> u32 {
        self.major_version
    }

    fn minor_version(&self) -> u32 {
        self.minor_version
    }

    fn array_uuid(&self) -> ArrayUuid {
        if self.minor_version < 90 {
            ArrayUuid::from_u32(self.array_uuid_0)
        } else {
            ArrayUuid::from_u32_4(&[
                self.array_uuid_0,
                self.array_uuid_1,
                self.array_uuid_2,
                self.array_uuid_3,
            ])
        }
    }

    fn array_name(&self) -> Option<&OsStr> {
        None
    }

    fn algorithm(&self) -> MdAlgorithm {
        MdAlgorithm::from_level_and_layout(self.level, self.layout)
    }

    fn size(&self) -> u64 {
        self.size.into()
    }

    fn chunk_size(&self) -> u32 {
        self.chunk_size
    }

    fn raid_disks(&self) -> u32 {
        self.raid_disks
    }

    fn reshape_status(&self) -> ReshapeStatus {
        self.reshape_status.clone()
    }

    fn event_count(&self) -> u64 {
        self.event_count
    }

    fn device_roles(&self) -> Vec<MdDeviceRole> {
        self.disks
            .iter()
            .map(|descriptor| descriptor.role)
            .collect()
    }
}
