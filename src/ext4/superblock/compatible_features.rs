use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct CompatibleFeatures : u32 {
        const DIRECTORY_PREALLOCATION = 1;
        const MAGIC_INODES = 2;
        const HAS_JOURNAL = 4;
        const SUPPORTS_EXTENDED_ATTRIBUTES = 8;
        const HAS_RESERVED_GDT_BLOCKS = 0x10;
        const HAS_DIRECTORY_INDICES = 0x20;
        const LAZY_BG = 0x40;
        const EXCLUDE_INODE = 0x80;
        const EXCLUDE_BITMAP = 0x100;
        const SPARSE_SUPERBLOCK_V2 = 0x200;
        const SUPPORTS_FAST_COMMITS = 0x400;
        const ORPHAN_PRESENT = 0x1000;
    }
}

impl LayoutAs<u32> for CompatibleFeatures {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.bits())
    }
}
