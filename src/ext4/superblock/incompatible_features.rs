use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct IncompatibleFeatures: u32 {
        const COMPRESSION = 1;
        const DIRECTORY_ENTRIES_RECORD_FILE_TYPE = 2;
        const NEEDS_RECOVERY = 4;
        const HAS_SEPARATE_JOURNAL_DEVICE = 8;
        const HAS_META_BLOCK_GROUPS = 0x10;
        const FILES_USE_EXTENTS = 0x40;
        const IS_64_BIT = 0x80;
        const MULTIPLE_MOUNT_PROTECTION = 0x100;
        const FLEXIBLE_BLOCK_GROUPS = 0x200;
        const INODES_STORE_LARGE_EXTENDED_ATTRIBUTE_VALUES = 0x400;
        const DATA_IN_DIRECTORY_ENTRY = 0x1000;

        /// If set, the metadata checksum seed is stored in the superblock.
        const METADATA_CHECKSUM_SEED = 0x2000;

        /// If set, the filesystem supports large directories >2GB or with
        /// 3-level htrees.
        const LARGE_DIRECTORIES = 0x4000;

        const DATA_IN_INODE = 0x8000;
        const HAS_ENCRYPTED_INODES = 0x10000;
    }
}

impl LayoutAs<u32> for IncompatibleFeatures {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.bits())
    }
}
