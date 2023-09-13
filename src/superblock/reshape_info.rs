use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};
use std::mem::size_of;

pub struct ReshapeInfo<'superblock>(&'superblock [u8; ReshapeInfo::LENGTH]);

impl<'superblock> ReshapeInfo<'superblock> {
    const NEW_LEVEL_OFFSET: usize = 0;
    const NEW_LEVEL_LENGTH: usize = size_of::<u32>();
    const NEW_LEVEL_END: usize = Self::NEW_LEVEL_OFFSET + Self::NEW_LEVEL_LENGTH;
    const RESHAPE_POSITION_OFFSET: usize = Self::NEW_LEVEL_END;
    const RESHAPE_POSITION_LENGTH: usize = size_of::<u64>();
    const RESHAPE_POSITION_END: usize =
        Self::RESHAPE_POSITION_OFFSET + Self::RESHAPE_POSITION_LENGTH;
    const DELTA_DISKS_OFFSET: usize = Self::RESHAPE_POSITION_END;
    const DELTA_DISKS_LENGTH: usize = size_of::<u32>();
    const DELTA_DISKS_END: usize = Self::DELTA_DISKS_OFFSET + Self::DELTA_DISKS_LENGTH;
    const NEW_LAYOUT_OFFSET: usize = Self::DELTA_DISKS_END;
    const NEW_LAYOUT_LENGTH: usize = size_of::<u32>();
    const NEW_LAYOUT_END: usize = Self::NEW_LAYOUT_OFFSET + Self::NEW_LAYOUT_LENGTH;
    const NEW_CHUNK_OFFSET: usize = Self::NEW_LAYOUT_END;
    const NEW_CHUNK_LENGTH: usize = size_of::<u32>();
    const NEW_CHUNK_END: usize = Self::NEW_CHUNK_OFFSET + Self::NEW_CHUNK_LENGTH;
    const NEW_OFFSET_OFFSET: usize = Self::NEW_CHUNK_END;
    const NEW_OFFSET_LENGTH: usize = size_of::<u32>();
    const NEW_OFFSET_END: usize = Self::NEW_OFFSET_OFFSET + Self::NEW_OFFSET_LENGTH;
    pub(super) const LENGTH: usize = Self::NEW_OFFSET_END;

    pub(super) fn new(buf: &'superblock [u8; ReshapeInfo::LENGTH]) -> Self {
        Self(buf)
    }

    pub fn new_level(&self) -> u32 {
        LittleEndian::read_u32(array_ref![
            self.0,
            Self::NEW_LEVEL_OFFSET,
            ReshapeInfo::NEW_LEVEL_LENGTH
        ])
    }
}
