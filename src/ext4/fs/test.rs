use crate::block_device::InMemoryBlockDevice;
use crate::ext::ReadAll;
use crate::ext4::fs::Ext4Fs;
use crate::ext4::inode::{FileMode, FileType, Permissions};
use crate::ext4::superblock::{
    CompatibleFeatures, CreatorOs, IncompatibleFeatures, ReadOnlyCompatibleFeatures,
};
use crate::ext4::{block_group, inode};
use chrono::{DateTime, NaiveDate, NaiveTime};
use flate2::read::GzDecoder;
use std::io;

fn zero_32mb_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("../test_data/zero-32MB.gz");
    Ok(InMemoryBlockDevice::new(
        GzDecoder::new(GZIPPED).read_all()?,
    ))
}

fn random_2mb_zero_30mb_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("../test_data/random-2MB-zero-30MB.gz");
    Ok(InMemoryBlockDevice::new(
        GzDecoder::new(GZIPPED).read_all()?,
    ))
}

fn ext4_100mb_empty_device() -> io::Result<InMemoryBlockDevice> {
    static GZIPPED: &[u8] = include_bytes!("../test_data/ext4-100MB-empty.gz");
    Ok(InMemoryBlockDevice::new(
        GzDecoder::new(GZIPPED).read_all()?,
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
    assert_eq!(fs.superblock.inodes_count(), 25600);
    assert_eq!(fs.superblock.blocks_count(), 25600);
    assert_eq!(fs.superblock.reserved_blocks_count(), 1280);
    assert_eq!(fs.superblock.free_blocks_count(), 22954);
    assert_eq!(fs.superblock.free_inodes_count(), 25589);
    assert_eq!(fs.superblock.first_data_block(), 0);
    assert_eq!(fs.superblock.block_size_bytes(), 4096);
    assert_eq!(fs.superblock.cluster_size_blocks(), 4096);
    assert_eq!(fs.superblock.blocks_per_group(), 32768);
    assert_eq!(fs.superblock.clusters_per_group(), 32768);
    assert_eq!(fs.superblock.inodes_per_group(), 25600);
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
    assert_eq!(fs.group_descriptors[0].block_bitmap_block(), 0xe);
    assert_eq!(fs.group_descriptors[0].inode_bitmap_block(), 0x1e);
    assert_eq!(fs.group_descriptors[0].inode_table_block(), 0x2e);
    assert_eq!(fs.group_descriptors[0].free_block_count(), 22954);
    assert_eq!(fs.group_descriptors[0].free_inode_count(), 25589);
    assert_eq!(fs.group_descriptors[0].used_directories_count(), 2);
    assert_eq!(
        fs.group_descriptors[0].flags(),
        block_group::Flags::INODE_TABLE_ZEROED
    );
    assert_eq!(fs.group_descriptors[0].exclude_bitmap_block(), 0);
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
    assert_eq!(inode.block_count(), 8);
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
