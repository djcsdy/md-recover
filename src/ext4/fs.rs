use crate::block_device::BlockDevice;
use crate::ext::LongMul;
use crate::ext4::block_group::BlockGroupDescriptor;
use crate::ext4::file::Ext4File;
use crate::ext4::inode::Inode;
use crate::ext4::superblock::{CreatorOs, IncompatibleFeatures, Superblock};
use crate::ext4::units::{FsBlockNumber, InodeCount, InodeNumber};
use bitflags::Flags;
use std::io;
use std::io::SeekFrom;
use std::rc::Rc;

#[derive(Clone)]
pub struct Ext4Fs<D: BlockDevice> {
    device: D,
    pub(super) superblock: Rc<Superblock<Vec<u8>>>,
    pub(super) group_descriptors: Rc<Vec<BlockGroupDescriptor>>,
}

impl<D: BlockDevice> Ext4Fs<D> {
    pub fn open(mut device: D) -> io::Result<Self> {
        device.seek(SeekFrom::Start(1024))?;
        let superblock = Superblock::read(&mut device)?;

        let block_count = superblock.blocks_count();
        let blocks_per_group = u64::from(superblock.blocks_per_group());
        let group_count = usize::try_from(block_count.div_ceil(blocks_per_group))
            .or(Err(io::ErrorKind::Unsupported))?;
        let group_size = usize::from(superblock.group_descriptor_size());

        if blocks_per_group == 0
            || group_size == 0
            || superblock.inodes_per_group() == InodeCount(0)
            || superblock.creator_os() != CreatorOs::Linux
            || !superblock
                .incompatible_features()
                .contains(IncompatibleFeatures::FILES_USE_EXTENTS)
            || superblock.incompatible_features().contains_unknown_bits()
        {
            return Err(io::ErrorKind::Unsupported.into());
        }

        let group_descriptors_block_number = u64::from(superblock.first_data_block()) + 1;
        device.seek(SeekFrom::Start(
            group_descriptors_block_number * superblock.block_size_bytes(),
        ))?;
        let mut group_descriptors = Vec::with_capacity(group_count);
        for _ in 0..group_count {
            let mut buf = vec![0u8; group_size];
            device.read_exact(&mut buf)?;
            group_descriptors.push(BlockGroupDescriptor::new(buf))
        }

        Ok(Self {
            device,
            superblock: Rc::new(superblock),
            group_descriptors: Rc::new(group_descriptors),
        })
    }

    pub fn block_size(&self) -> usize {
        usize::try_from(self.superblock.block_size_bytes()).unwrap()
    }

    pub fn read_root_inode(&mut self) -> io::Result<Inode> {
        self.read_inode(InodeNumber(2))
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            device: self.device.try_clone()?,
            superblock: self.superblock.clone(),
            group_descriptors: self.group_descriptors.clone(),
        })
    }

    fn read_inode(&mut self, inode_number: InodeNumber) -> io::Result<Inode> {
        if inode_number == InodeNumber(0) || inode_number > self.superblock.inodes_count() {
            return Err(io::ErrorKind::InvalidInput.into());
        }

        let group_index = (*inode_number - 1) / *self.superblock.inodes_per_group();
        let index_in_group = (*inode_number - 1) % *self.superblock.inodes_per_group();

        let group = self
            .group_descriptors
            .get(usize::try_from(group_index).or(Err(io::ErrorKind::InvalidInput))?)
            .ok_or(io::ErrorKind::InvalidInput)?;

        let inode_offset_within_table =
            index_in_group.long_mul(u32::from(self.superblock.inode_size()));
        let inode_byte_offset = group
            .inode_table_block()
            .checked_mul(self.superblock.block_size_bytes())
            .and_then(|base| base.checked_add(inode_offset_within_table))
            .ok_or(io::ErrorKind::InvalidData)?;

        self.device.seek(SeekFrom::Start(inode_byte_offset))?;
        let mut buffer = vec![0; usize::from(self.superblock.inode_size())];
        self.device.read_exact(&mut buffer)?;
        Ok(Inode::new(&self.superblock, inode_number, buffer))
    }

    pub(super) fn read_block(
        &mut self,
        block_number: FsBlockNumber,
        buf: &mut [u8],
    ) -> io::Result<()> {
        if buf.len() < self.block_size() {
            Err(io::ErrorKind::InvalidInput.into())
        } else {
            self.device.seek(SeekFrom::Start(
                *block_number * self.superblock.block_size_bytes(),
            ))?;
            self.device.read_exact(buf)?;
            Ok(())
        }
    }

    pub fn open_file(&mut self, inode_number: InodeNumber) -> io::Result<Ext4File<D>> {
        Ext4File::open(self.try_clone()?, self.read_inode(inode_number)?)
    }
}

#[cfg(test)]
pub(in crate::ext4) mod test {
    use crate::block_device::{BlockSize, InMemoryBlockDevice};
    use crate::ext::ReadAll;
    use crate::ext4::fs::Ext4Fs;
    use crate::ext4::inode::{FileMode, FileType, Permissions};
    use crate::ext4::superblock::{
        CompatibleFeatures, CreatorOs, IncompatibleFeatures, ReadOnlyCompatibleFeatures,
    };
    use crate::ext4::units::{BlockCount, FsBlockNumber, InodeCount};
    use crate::ext4::{block_group, inode};
    use chrono::{DateTime, NaiveDate, NaiveTime};
    use flate2::read::GzDecoder;
    use std::io;

    pub(in crate::ext4) fn zero_32mb_device() -> io::Result<InMemoryBlockDevice> {
        static GZIPPED: &[u8] = include_bytes!("test_data/zero-32MB.gz");
        Ok(InMemoryBlockDevice::new(
            GzDecoder::new(GZIPPED).read_all()?,
            BlockSize::default(),
        ))
    }

    pub(in crate::ext4) fn random_2mb_zero_30mb_device() -> io::Result<InMemoryBlockDevice> {
        static GZIPPED: &[u8] = include_bytes!("test_data/random-2MB-zero-30MB.gz");
        Ok(InMemoryBlockDevice::new(
            GzDecoder::new(GZIPPED).read_all()?,
            BlockSize::default(),
        ))
    }

    pub(in crate::ext4) fn ext4_100mb_empty_device() -> io::Result<InMemoryBlockDevice> {
        static GZIPPED: &[u8] = include_bytes!("test_data/ext4-100MB-empty.gz");
        Ok(InMemoryBlockDevice::new(
            GzDecoder::new(GZIPPED).read_all()?,
            BlockSize::default(),
        ))
    }

    #[test]
    fn open_zero_32mb() -> anyhow::Result<()> {
        assert_eq!(
            Ext4Fs::open(zero_32mb_device()?)
                .err()
                .map(|error| error.kind()),
            Some(io::ErrorKind::InvalidData)
        );
        Ok(())
    }

    #[test]
    fn open_random_2mb_zero_30mb() -> anyhow::Result<()> {
        assert_eq!(
            Ext4Fs::open(random_2mb_zero_30mb_device()?)
                .err()
                .map(|error| error.kind()),
            Some(io::ErrorKind::InvalidData)
        );
        Ok(())
    }

    #[test]
    fn open_100mb_empty() -> anyhow::Result<()> {
        let fs = Ext4Fs::open(ext4_100mb_empty_device()?)?;
        assert!(fs.superblock.valid());
        assert_eq!(fs.superblock.inodes_count(), InodeCount(25600));
        assert_eq!(fs.superblock.blocks_count(), BlockCount(25600));
        assert_eq!(fs.superblock.reserved_blocks_count(), BlockCount(1280));
        assert_eq!(fs.superblock.free_blocks_count(), BlockCount(22954));
        assert_eq!(fs.superblock.free_inodes_count(), InodeCount(25589));
        assert_eq!(fs.superblock.first_data_block(), FsBlockNumber(0));
        assert_eq!(fs.superblock.block_size_bytes(), 4096);
        assert_eq!(fs.superblock.cluster_size_blocks(), 4096);
        assert_eq!(fs.superblock.blocks_per_group(), 32768);
        assert_eq!(fs.superblock.clusters_per_group(), 32768);
        assert_eq!(fs.superblock.inodes_per_group(), InodeCount(25600));
        assert_eq!(fs.superblock.creator_os(), CreatorOs::Linux);
        assert_eq!(fs.superblock.revision_level(), 1);
        assert_eq!(fs.superblock.first_inode(), 11);
        assert_eq!(fs.superblock.inode_size(), 256);
        assert_eq!(fs.superblock.block_group_number(), 0);
        assert_eq!(
            fs.superblock.compatible_features(),
            CompatibleFeatures::HAS_JOURNAL
                | CompatibleFeatures::SUPPORTS_EXTENDED_ATTRIBUTES
                | CompatibleFeatures::HAS_RESERVED_GDT_BLOCKS
                | CompatibleFeatures::HAS_DIRECTORY_INDICES
        );
        assert_eq!(
            fs.superblock.incompatible_features(),
            IncompatibleFeatures::DIRECTORY_ENTRIES_RECORD_FILE_TYPE
                | IncompatibleFeatures::FILES_USE_EXTENTS
                | IncompatibleFeatures::IS_64_BIT
                | IncompatibleFeatures::FLEXIBLE_BLOCK_GROUPS
        );
        assert_eq!(
            fs.superblock.read_only_compatible_features(),
            ReadOnlyCompatibleFeatures::SPARSE_SUPERBLOCKS
                | ReadOnlyCompatibleFeatures::CONTAINS_LARGE_FILES
                | ReadOnlyCompatibleFeatures::CONTAINS_HUGE_FILES
                | ReadOnlyCompatibleFeatures::UNLIMITED_SUBDIRECTORIES
                | ReadOnlyCompatibleFeatures::CONTAINS_LARGE_INODES
                | ReadOnlyCompatibleFeatures::METADATA_CHECKSUMS
        );
        assert_eq!(fs.group_descriptors.len(), 1);
        assert_eq!(
            fs.group_descriptors[0].block_bitmap_block(),
            FsBlockNumber(0xe)
        );
        assert_eq!(
            fs.group_descriptors[0].inode_bitmap_block(),
            FsBlockNumber(0x1e)
        );
        assert_eq!(
            fs.group_descriptors[0].inode_table_block(),
            FsBlockNumber(0x2e)
        );
        assert_eq!(
            fs.group_descriptors[0].free_block_count(),
            BlockCount(22954)
        );
        assert_eq!(fs.group_descriptors[0].free_inode_count(), 25589);
        assert_eq!(fs.group_descriptors[0].used_directories_count(), 2);
        assert_eq!(
            fs.group_descriptors[0].flags(),
            block_group::Flags::INODE_TABLE_ZEROED
        );
        assert_eq!(
            fs.group_descriptors[0].exclude_bitmap_block(),
            FsBlockNumber(0)
        );
        assert_eq!(fs.group_descriptors[0].block_bitmap_checksum(), 0x30609723);
        assert_eq!(fs.group_descriptors[0].inode_bitmap_checksum(), 0x957502e9);
        assert_eq!(fs.group_descriptors[0].unused_inode_count(), 0x63f5);
        assert_eq!(fs.group_descriptors[0].checksum(), 0x67e2);
        Ok(())
    }

    #[test]
    fn read_root_inode() -> anyhow::Result<()> {
        let mut fs = Ext4Fs::open(ext4_100mb_empty_device()?)?;
        let inode = fs.read_root_inode()?;
        assert_eq!(
            inode.file_mode(),
            FileMode::from_file_type_and_permissions(
                FileType::Directory,
                Permissions::USER_READ
                    | Permissions::USER_WRITE
                    | Permissions::USER_EXECUTE
                    | Permissions::GROUP_READ
                    | Permissions::GROUP_EXECUTE
                    | Permissions::OTHER_READ
                    | Permissions::OTHER_EXECUTE
            )
        );
        assert_eq!(inode.owner_user_id(), 0);
        assert_eq!(inode.file_size_bytes(), 4096);
        assert_eq!(
            inode.access_time(),
            NaiveDate::from_ymd_opt(2025, 11, 11)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(13, 34, 21).unwrap())
                .and_utc()
        );
        assert_eq!(
            inode.change_time(),
            NaiveDate::from_ymd_opt(2025, 11, 11)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(13, 34, 21).unwrap())
                .and_utc()
        );
        assert_eq!(
            inode.modified_time(),
            NaiveDate::from_ymd_opt(2025, 11, 11)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(13, 34, 21).unwrap())
                .and_utc()
        );
        assert_eq!(inode.delete_time(), DateTime::UNIX_EPOCH);
        assert_eq!(inode.group_id(), 0);
        assert_eq!(inode.links_count(), 3);
        assert_eq!(inode.block_count(), BlockCount(8));
        assert_eq!(inode.flags(), inode::Flags::HAS_EXTENTS);
        assert_eq!(inode.version(), 0);
        assert_eq!(
            inode.blocks_buffer().as_slice(),
            &[
                0x0a, 0xf3, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]
        );
        assert_eq!(inode.generation(), 0);
        assert_eq!(inode.file_acl(), 0);
        assert_eq!(inode.checksum(), 0xDA6E700E);
        assert_eq!(
            inode.creation_time(),
            NaiveDate::from_ymd_opt(2025, 11, 11)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(13, 34, 21).unwrap())
                .and_utc()
        );
        assert_eq!(inode.project_id(), 0);
        Ok(())
    }
}
