use std::mem::size_of;

pub struct PplInfo<'superblock>(&'superblock [u8; PplInfo::LENGTH]);

impl<'superblock> PplInfo<'superblock> {
    const OFFSET_OFFSET: usize = 0;
    const OFFSET_LENGTH: usize = size_of::<i16>();
    const OFFSET_END: usize = Self::OFFSET_OFFSET + Self::OFFSET_LENGTH;
    const SIZE_OFFSET: usize = Self::OFFSET_END;
    const SIZE_LENGTH: usize = size_of::<u16>();
    const SIZE_END: usize = Self::SIZE_OFFSET + Self::SIZE_LENGTH;
    pub(super) const LENGTH: usize = Self::SIZE_END;

    pub(super) fn new(buf: &'superblock [u8; PplInfo::LENGTH]) -> Self {
        Self(buf)
    }
}
