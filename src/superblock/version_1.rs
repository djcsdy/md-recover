use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};
use std::io::{Error, ErrorKind, Result};
use std::mem::size_of;

const MAGIC_OFFSET: usize = 0;
const MAGIC_LENGTH: usize = size_of::<u32>();
const MAGIC_END: usize = MAGIC_OFFSET + MAGIC_LENGTH;
const MAJOR_VERSION_OFFSET: usize = MAGIC_END;
const MAJOR_VERSION_LENGTH: usize = size_of::<u32>();
const MAJOR_VERSION_END: usize = MAJOR_VERSION_OFFSET + MAJOR_VERSION_LENGTH;
const FEATURE_MAP_OFFSET: usize = MAJOR_VERSION_END;
const FEATURE_MAP_LENGTH: usize = size_of::<u32>();
const FEATURE_MAP_END: usize = FEATURE_MAP_OFFSET + FEATURE_MAP_LENGTH;
const PAD_0_OFFSET: usize = FEATURE_MAP_END;
const PAD_0_END: usize = PAD_0_OFFSET + size_of::<u32>();
const ARRAY_UUID_OFFSET: usize = PAD_0_END;
const ARRAY_UUID_LENGTH: usize = 16 * size_of::<u8>();
const ARRAY_UUID_END: usize = ARRAY_UUID_OFFSET + ARRAY_UUID_LENGTH;
const SET_NAME_OFFSET: usize = ARRAY_UUID_END;
const SET_NAME_LENGTH: usize = 32;
const SET_NAME_END: usize = SET_NAME_OFFSET + size_of::<u8>() * SET_NAME_LENGTH;
const CTIME_OFFSET: usize = SET_NAME_END;
const CTIME_END: usize = CTIME_OFFSET + size_of::<u64>();
const LEVEL_OFFSET: usize = CTIME_END;
const LEVEL_END: usize = LEVEL_OFFSET + size_of::<u32>();
const LAYOUT_OFFSET: usize = LEVEL_END;
const LAYOUT_END: usize = LAYOUT_OFFSET + size_of::<u32>();
const SIZE_OFFSET: usize = LAYOUT_END;
const SIZE_END: usize = SIZE_OFFSET + size_of::<u64>();
const CHUNK_SIZE_OFFSET: usize = SIZE_END;
const CHUNK_SIZE_END: usize = CHUNK_SIZE_OFFSET + size_of::<u32>();
const RAID_DISKS_OFFSET: usize = CHUNK_SIZE_END;
const RAID_DISKS_END: usize = RAID_DISKS_OFFSET + size_of::<u32>();
const BITMAP_OFFSET_OFFSET: usize = RAID_DISKS_END;
const BITMAP_OFFSET_END: usize = BITMAP_OFFSET_OFFSET + size_of::<u32>();
const PPL_OFFSET_OFFSET: usize = RAID_DISKS_END;
const PPL_OFFSET_END: usize = PPL_OFFSET_OFFSET + size_of::<i16>();
const PPL_SIZE_OFFSET: usize = PPL_OFFSET_END;
const PPL_SIZE_END: usize = PPL_SIZE_OFFSET + size_of::<u16>();
const NEW_LEVEL_OFFSET: usize = RAID_DISKS_END + size_of::<u32>();
const NEW_LEVEL_END: usize = NEW_LEVEL_OFFSET + size_of::<u32>();
const RESHAPE_POSITION_OFFSET: usize = NEW_LEVEL_END;
const RESHAPE_POSITION_END: usize = RESHAPE_POSITION_OFFSET + size_of::<u64>();
const DELTA_DISKS_OFFSET: usize = RESHAPE_POSITION_END;
const DELTA_DISKS_END: usize = DELTA_DISKS_OFFSET + size_of::<u32>();
const NEW_LAYOUT_OFFSET: usize = DELTA_DISKS_END;
const NEW_LAYOUT_END: usize = NEW_LAYOUT_OFFSET + size_of::<u32>();
const NEW_CHUNK_SIZE_OFFSET: usize = NEW_LAYOUT_END;
const NEW_CHUNK_SIZE_END: usize = NEW_CHUNK_SIZE_OFFSET + size_of::<u32>();
const NEW_OFFSET_OFFSET: usize = NEW_CHUNK_SIZE_END;
const NEW_OFFSET_END: usize = NEW_OFFSET_OFFSET + size_of::<u32>();
const DATA_OFFSET_OFFSET: usize = NEW_OFFSET_END;
const DATA_OFFSET_END: usize = DATA_OFFSET_OFFSET + size_of::<u64>();
const DATA_SIZE_OFFSET: usize = DATA_OFFSET_END;
const DATA_SIZE_END: usize = DATA_SIZE_OFFSET + size_of::<u64>();
const SUPER_OFFSET_OFFSET: usize = DATA_SIZE_END;
const SUPER_OFFSET_END: usize = SUPER_OFFSET_OFFSET + size_of::<u64>();
const RECOVERY_OFFSET_OFFSET: usize = SUPER_OFFSET_END;
const RECOVERY_OFFSET_END: usize = RECOVERY_OFFSET_OFFSET + size_of::<u64>();
const JOURNAL_TAIL_OFFSET: usize = SUPER_OFFSET_END;
const JOURNAL_TAIL_END: usize = JOURNAL_TAIL_OFFSET + size_of::<u64>();
const DEV_NUMBER_OFFSET: usize = SUPER_OFFSET_END + size_of::<u64>();
const DEV_NUMBER_END: usize = DEV_NUMBER_OFFSET + size_of::<u32>();
const COUNT_CORRECTED_READ_OFFSET: usize = DEV_NUMBER_END;
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

    fn feature_map(&self) -> u32 {
        LittleEndian::read_u32(array_ref![self.0, FEATURE_MAP_OFFSET, FEATURE_MAP_LENGTH])
    }

    fn array_uuid(&self) -> &[u8; ARRAY_UUID_LENGTH] {
        array_ref![self.0, ARRAY_UUID_OFFSET, ARRAY_UUID_LENGTH]
    }
}
