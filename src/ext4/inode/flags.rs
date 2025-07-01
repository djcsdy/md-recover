use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct Flags: u32 {
        const SECURE_DELETE = 1;
        const UNDELETE = 2;
        const COMPRESSED = 4;
        const SYNCHRONOUS_UPDATES = 8;
        const IMMUTABLE = 0x10;
        const APPEND_ONLY = 0x20;
        const NO_DUMP = 0x40;
        const NO_UPDATE_ACCESS_TIME = 0x80;
        const DIRTY = 0x100;
        const COMPRESSED_CLUSTERS = 0x200;
        const NO_COMPRESS = 0x400;
        const ENCRYPTED = 0x800;
        const HASH_INDEXED_DIRECTORY = 0x1000;
        const AFS_DIRECTORY = 0x2000;
        const JOURNAL_DATA = 0x4000;
        const NO_TAIL_MERGE = 0x8000;
        const DIRECTORY_SYNC = 0x10000;
        const TOP_DIRECTORY = 0x20000;
        const HUGE_FILE = 0x40000;
        const HAS_EXTENTS = 0x80000;
        const VERITY_PROTECTED = 0x100000;
        const LARGE_EXTENDED_ATTRIBUTES = 0x200000;
        const DIRECT_ACCESS = 0x2000000;
        const HAS_INLINE_DATA = 0x10000000;
        const INHERIT_PROJECT_ID = 0x20000000;
        const CASE_FOLDED = 0x40000000;
        const RESERVED = 0x80000000;
    }
}

impl LayoutAs<u32> for Flags {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.bits())
    }
}
