use crate::block_device::InMemoryBlockDevice;
use crate::ext::ReadAll;
use crate::ext4::block_group::{BlockGroupDescriptor, Flags};
use crate::ext4::fs::Ext4Fs;
use crate::ext4::superblock::{
    CompatibleFeatures, CreatorOs, IncompatibleFeatures, ReadOnlyCompatibleFeatures,
};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;
use std::{fs, io};

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
    assert_eq!(
        fs.group_descriptors,
        vec![BlockGroupDescriptor {
            block_bitmap_block: 0xe,
            inode_bitmap_block: 0x1e,
            inode_table_block: 0x2e,
            free_block_count: 22954,
            free_inode_count: 25589,
            used_directories_count: 2,
            flags: Flags::INODE_TABLE_ZEROED,
            exclude_bitmap_block: 0,
            block_bitmap_checksum: 0x30609723,
            inode_bitmap_checksum: 0x957502e9,
            unused_inode_count: 0x63f5,
            checksum: 0x67e2,
        }]
    );
    Ok(())
}
