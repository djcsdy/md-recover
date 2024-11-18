use binary_layout::LayoutAs;
use std::convert::Infallible;

bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct ReadOnlyCompatibleFeatures: u32 {
        const SPARSE_SUPERBLOCKS = 1;

        /// If set, the filesystem has been used to store a file larger than 2GiB.
        const CONTAINS_LARGE_FILES = 2;

        /// Unknown, not used by Linux.
        const BTREE_DIR = 4;

        /// If set, the filesystem has been used to store a huge file whose
        /// size is represented in units of logical blocks instead of in
        /// 512-byte sectors.
        const CONTAINS_HUGE_FILES = 8;

        /// If set, Group Descriptors have checksums.
        const GROUP_DESCRIPTORS_HAVE_CHECKSUMS = 0x10;

        /// If set, the old ext3 32,000 subdirectory limit does not apply to
        /// this filesystem.
        const UNLIMITED_SUBDIRECTORIES = 0x20;

        /// If set, large inodes exist on this filesystem.
        const CONTAINS_LARGE_INODES = 0x40;

        /// If set, the filesystem has a snapshot.
        const HAS_SNAPSHOT = 0x80;

        /// Unknown, relates to quota feature.
        const QUOTA = 0x100;

        /// If set, the filesystem supports "bigalloc", which means that file
        /// extents are tracked in units of clusters instead of units of
        /// blocks.
        const BIGALLOC = 0x200;

        /// If set, the filesystem supports metadata checksumming.
        const METADATA_CHECKSUMS = 0x400;

        /// If set, the filesystem supports replicas.
        const REPLICA = 0x800;

        /// If set, the filesystem may only be mounted read-only.
        const READ_ONLY = 0x1000;

        /// If set, the filesystem tracks project quotas.
        const PROJECT_QUOTA = 0x2000;

        /// If set, verity inodes may be present on the filesystem;
        const VERITY = 0x8000;

        /// If set, orphan files may have valid orphan entries, which need to
        /// be cleaned up when mounting the filesystem.
        const ORPHAN_PRESENT = 0x10000;
    }
}

impl LayoutAs<u32> for ReadOnlyCompatibleFeatures {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self::from_bits_retain(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.bits())
    }
}
