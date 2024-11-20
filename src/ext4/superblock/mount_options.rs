use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct MountOptions: u32 {
        const PRINT_DEBUGGING_INFO = 1;
        const NEW_FILES_INHERIT_DIRECTORY_GROUP_ID = 2;
        const ENABLE_USERSPACE_EXTENDED_ATTRIBUTES = 4;
        const ENABLE_POSIX_ACLS = 8;
        const DISABLE_16_BIT_USER_IDS = 0x10;
        const COMMIT_ALL_DATA_AND_METADATA_TO_JOURNAL = 0x20;
        const FLUSH_ALL_DATA_BEFORE_COMMITTING_METADATA = 0x40;
        const DATA_ORDERING_NOT_PRESERVED = 0x80;
        const DISABLE_WRITE_FLUSHES = 0x100;
        const TRACK_METADATA_BLOCKS = 0x200;
        const ENABLE_DISCARD = 0x400;
        const DISABLE_DELAYED_ALLOCATION = 0x800;
    }
}

impl LayoutAs<u32> for MountOptions {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.bits())
    }
}
