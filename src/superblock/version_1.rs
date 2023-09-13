use crate::superblock::features::Features;
use crate::superblock::ppl_info::PplInfo;
use crate::superblock::reshape_info::ReshapeInfo;
use arrayref::array_ref;
use bitflags::Flags;
use byteorder::{ByteOrder, LittleEndian};
use std::io::{Error, ErrorKind, Result};
use std::mem::size_of;

const MAGIC_OFFSET: usize = 0;
const MAGIC_LENGTH: usize = size_of::<u32>();
const MAGIC_END: usize = MAGIC_OFFSET + MAGIC_LENGTH;
const MAJOR_VERSION_OFFSET: usize = MAGIC_END;
const MAJOR_VERSION_LENGTH: usize = size_of::<u32>();
const MAJOR_VERSION_END: usize = MAJOR_VERSION_OFFSET + MAJOR_VERSION_LENGTH;
const FEATURES_OFFSET: usize = MAJOR_VERSION_END;
const FEATURES_LENGTH: usize = size_of::<u32>();
const FEATURES_END: usize = FEATURES_OFFSET + FEATURES_LENGTH;
const PAD_0_OFFSET: usize = FEATURES_END;
const PAD_0_END: usize = PAD_0_OFFSET + size_of::<u32>();
const ARRAY_UUID_OFFSET: usize = PAD_0_END;
const ARRAY_UUID_LENGTH: usize = 16 * size_of::<u8>();
const ARRAY_UUID_END: usize = ARRAY_UUID_OFFSET + ARRAY_UUID_LENGTH;
const ARRAY_NAME_OFFSET: usize = ARRAY_UUID_END;
const ARRAY_NAME_LENGTH: usize = 32 * size_of::<u8>();
const ARRAY_NAME_END: usize = ARRAY_NAME_OFFSET + ARRAY_NAME_LENGTH;
const CTIME_OFFSET: usize = ARRAY_NAME_END;
const CTIME_LENGTH: usize = size_of::<u64>();
const CTIME_END: usize = CTIME_OFFSET + CTIME_LENGTH;
const LEVEL_OFFSET: usize = CTIME_END;
const LEVEL_LENGTH: usize = size_of::<u32>();
const LEVEL_END: usize = LEVEL_OFFSET + LEVEL_LENGTH;
const LAYOUT_OFFSET: usize = LEVEL_END;
const LAYOUT_LENGTH: usize = size_of::<u32>();
const LAYOUT_END: usize = LAYOUT_OFFSET + LAYOUT_LENGTH;
const SIZE_OFFSET: usize = LAYOUT_END;
const SIZE_LENGTH: usize = size_of::<u64>();
const SIZE_END: usize = SIZE_OFFSET + SIZE_LENGTH;
const CHUNK_SIZE_OFFSET: usize = SIZE_END;
const CHUNK_SIZE_LENGTH: usize = size_of::<u32>();
const CHUNK_SIZE_END: usize = CHUNK_SIZE_OFFSET + CHUNK_SIZE_LENGTH;
const RAID_DISKS_OFFSET: usize = CHUNK_SIZE_END;
const RAID_DISKS_LENGTH: usize = size_of::<u32>();
const RAID_DISKS_END: usize = RAID_DISKS_OFFSET + RAID_DISKS_LENGTH;
const BITMAP_OFFSET_OFFSET: usize = RAID_DISKS_END;
const BITMAP_OFFSET_LENGTH: usize = size_of::<u32>();
const BITMAP_OFFSET_END: usize = BITMAP_OFFSET_OFFSET + BITMAP_OFFSET_LENGTH;
const PPL_INFO_OFFSET: usize = RAID_DISKS_END;
const PPL_INFO_LENGTH: usize = PplInfo::LENGTH;
const PPL_INFO_END: usize = PPL_INFO_OFFSET + PPL_INFO_LENGTH;
const RESHAPE_INFO_OFFSET: usize = RAID_DISKS_END + size_of::<u32>();
const RESHAPE_INFO_LENGTH: usize = ReshapeInfo::LENGTH;
const RESHAPE_INFO_END: usize = RESHAPE_INFO_OFFSET + RESHAPE_INFO_LENGTH;
const DATA_OFFSET_OFFSET: usize = RESHAPE_INFO_END;
const DATA_OFFSET_LENGTH: usize = size_of::<u64>();
const DATA_OFFSET_END: usize = DATA_OFFSET_OFFSET + DATA_OFFSET_LENGTH;
const DATA_SIZE_OFFSET: usize = DATA_OFFSET_END;
const DATA_SIZE_LENGTH: usize = size_of::<u64>();
const DATA_SIZE_END: usize = DATA_SIZE_OFFSET + DATA_SIZE_LENGTH;
const SUPER_OFFSET_OFFSET: usize = DATA_SIZE_END;
const SUPER_OFFSET_LENGTH: usize = size_of::<u64>();
const SUPER_OFFSET_END: usize = SUPER_OFFSET_OFFSET + SUPER_OFFSET_LENGTH;
const RECOVERY_OFFSET_OFFSET: usize = SUPER_OFFSET_END;
const RECOVERY_OFFSET_LENGTH: usize = size_of::<u64>();
const RECOVERY_OFFSET_END: usize = RECOVERY_OFFSET_OFFSET + RECOVERY_OFFSET_LENGTH;
const JOURNAL_TAIL_OFFSET: usize = SUPER_OFFSET_END;
const JOURNAL_TAIL_LENGTH: usize = size_of::<u64>();
const JOURNAL_TAIL_END: usize = JOURNAL_TAIL_OFFSET + JOURNAL_TAIL_LENGTH;
const DEVICE_NUMBER_OFFSET: usize = SUPER_OFFSET_END + size_of::<u64>();
const DEVICE_NUMBER_LENGTH: usize = size_of::<u32>();
const DEVICE_NUMBER_END: usize = DEVICE_NUMBER_OFFSET + DEVICE_NUMBER_LENGTH;
const COUNT_CORRECTED_READ_OFFSET: usize = DEVICE_NUMBER_END;
const COUNT_CORRECTED_READ_END: usize = COUNT_CORRECTED_READ_OFFSET + size_of::<u32>();
const DEVICE_UUID_OFFSET: usize = COUNT_CORRECTED_READ_END;
const DEVICE_UUID_LENGTH: usize = 16;
const DEVICE_UUID_END: usize = DEVICE_UUID_OFFSET + size_of::<u8>() * DEVICE_UUID_LENGTH;
const DEVICE_FLAGS_OFFSET: usize = DEVICE_UUID_END;
const DEVICE_FLAGS_END: usize = DEVICE_FLAGS_OFFSET + size_of::<u8>();
const BAD_BLOCK_LOG_SHIFT_OFFSET: usize = DEVICE_FLAGS_END;
const BAD_BLOCK_LOG_SHIFT_END: usize = BAD_BLOCK_LOG_SHIFT_OFFSET + size_of::<u8>();
const BAD_BLOCK_LOG_SIZE_OFFSET: usize = BAD_BLOCK_LOG_SHIFT_END;
const BAD_BLOCK_LOG_SIZE_END: usize = BAD_BLOCK_LOG_SIZE_OFFSET + size_of::<u16>();
const BAD_BLOCK_LOG_OFFSET_OFFSET: usize = BAD_BLOCK_LOG_SIZE_END;
const BAD_BLOCK_LOG_OFFSET_END: usize = BAD_BLOCK_LOG_OFFSET_OFFSET + size_of::<u32>();
const UTIME_OFFSET: usize = BAD_BLOCK_LOG_OFFSET_END;
const UTIME_END: usize = UTIME_OFFSET + size_of::<u64>();
const EVENTS_OFFSET: usize = UTIME_END;
const EVENTS_END: usize = EVENTS_OFFSET + size_of::<u64>();
const RESYNC_OFFSET_OFFSET: usize = EVENTS_END;
const RESYNC_OFFSET_END: usize = RESYNC_OFFSET_OFFSET + size_of::<u64>();
const SUPERBLOCK_CHECKSUM_OFFSET: usize = RESYNC_OFFSET_END;
const SUPERBLOCK_CHECKSUM_END: usize = SUPERBLOCK_CHECKSUM_OFFSET + size_of::<u32>();
const MAX_DEV_OFFSET: usize = SUPERBLOCK_CHECKSUM_END;
const MAX_DEV_END: usize = MAX_DEV_OFFSET + size_of::<u32>();
const PAD_3_OFFSET: usize = MAX_DEV_END;
const PAD_3_LENGTH: usize = 32;
const PAD_3_END: usize = PAD_3_OFFSET + size_of::<u8>() * PAD_3_LENGTH;
const DEV_ROLES_OFFSET: usize = PAD_3_END;

const MIN_SUPERBLOCK_LENGTH: usize = DEV_ROLES_OFFSET;
const MAX_SUPERBLOCK_LENGTH: usize = 4096;

pub struct SuperblockVersion1(Vec<u8>);

impl SuperblockVersion1 {
    pub fn read<B: Into<Vec<u8>>>(buffer: B) -> Result<Self> {
        let vec = buffer.into();
        if vec.len() < MIN_SUPERBLOCK_LENGTH {
            Err(Error::from(ErrorKind::UnexpectedEof))
        } else if Self::valid_magic(&vec) && Self::valid_major_version(&vec) {
            Ok(Self(vec))
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }

    fn valid_magic(buffer: &Vec<u8>) -> bool {
        LittleEndian::read_u32(array_ref![buffer, MAGIC_OFFSET, MAGIC_LENGTH]) == 0xa92b4efc
    }

    fn valid_major_version(buffer: &Vec<u8>) -> bool {
        LittleEndian::read_u32(array_ref![
            buffer,
            MAJOR_VERSION_OFFSET,
            MAJOR_VERSION_LENGTH
        ]) == 1
    }

    pub fn features(&self) -> Features {
        Features::from_bits(LittleEndian::read_u32(array_ref![
            self.0,
            FEATURES_OFFSET,
            FEATURES_LENGTH
        ]))
        .unwrap()
    }

    pub fn array_uuid(&self) -> &[u8; ARRAY_UUID_LENGTH] {
        array_ref![self.0, ARRAY_UUID_OFFSET, ARRAY_UUID_LENGTH]
    }

    pub fn array_name(&self) -> &[u8; ARRAY_NAME_LENGTH] {
        array_ref![self.0, ARRAY_NAME_OFFSET, ARRAY_NAME_LENGTH]
    }

    pub fn ctime(&self) -> u64 {
        LittleEndian::read_u64(array_ref![self.0, CTIME_OFFSET, CTIME_LENGTH])
    }

    pub fn level(&self) -> u32 {
        LittleEndian::read_u32(array_ref![self.0, LEVEL_OFFSET, LEVEL_LENGTH])
    }

    pub fn layout(&self) -> u32 {
        LittleEndian::read_u32(array_ref![self.0, LAYOUT_OFFSET, LAYOUT_LENGTH])
    }

    pub fn size(&self) -> u64 {
        LittleEndian::read_u64(array_ref![self.0, SIZE_OFFSET, SIZE_LENGTH])
    }

    pub fn chunk_size(&self) -> u32 {
        LittleEndian::read_u32(array_ref![self.0, CHUNK_SIZE_OFFSET, CHUNK_SIZE_LENGTH])
    }

    pub fn raid_disks(&self) -> u32 {
        LittleEndian::read_u32(array_ref![self.0, RAID_DISKS_OFFSET, RAID_DISKS_LENGTH])
    }

    pub fn bitmap_offset(&self) -> Option<u32> {
        if self.features().contains(Features::BITMAP_OFFSET) {
            Some(LittleEndian::read_u32(array_ref![
                self.0,
                BITMAP_OFFSET_OFFSET,
                BITMAP_OFFSET_LENGTH
            ]))
        } else {
            None
        }
    }

    pub fn ppl_info(&self) -> Option<PplInfo> {
        if self.features().contains(Features::PPL) {
            Some(PplInfo::new(array_ref![
                self.0,
                PPL_INFO_OFFSET,
                PPL_INFO_LENGTH
            ]))
        } else {
            None
        }
    }

    pub fn reshape_info(&self) -> Option<ReshapeInfo> {
        if self.features().contains(Features::RESHAPE_ACTIVE) {
            Some(ReshapeInfo::new(array_ref![
                self.0,
                RESHAPE_INFO_OFFSET,
                RESHAPE_INFO_LENGTH
            ]))
        } else {
            None
        }
    }

    pub fn data_offset(&self) -> u64 {
        LittleEndian::read_u64(array_ref![self.0, DATA_OFFSET_OFFSET, DATA_OFFSET_LENGTH])
    }

    pub fn data_size(&self) -> u64 {
        LittleEndian::read_u64(array_ref![self.0, DATA_SIZE_OFFSET, DATA_SIZE_LENGTH])
    }

    pub fn super_offset(&self) -> u64 {
        LittleEndian::read_u64(array_ref![self.0, SUPER_OFFSET_OFFSET, SUPER_OFFSET_LENGTH])
    }

    pub fn recovery_offset(&self) -> Option<u64> {
        if self.features().contains(Features::RECOVERY_OFFSET) {
            Some(LittleEndian::read_u64(array_ref![
                self.0,
                RECOVERY_OFFSET_OFFSET,
                RECOVERY_OFFSET_LENGTH
            ]))
        } else {
            None
        }
    }

    pub fn journal_tail(&self) -> Option<u64> {
        if self.features().contains(Features::JOURNAL) {
            Some(LittleEndian::read_u64(array_ref![
                self.0,
                JOURNAL_TAIL_OFFSET,
                JOURNAL_TAIL_LENGTH
            ]))
        } else {
            None
        }
    }

    pub fn device_number(&self) -> u32 {
        LittleEndian::read_u32(array_ref![
            self.0,
            DEVICE_NUMBER_OFFSET,
            DEVICE_NUMBER_LENGTH
        ])
    }
}
