use crate::ext4::superblock::state::State;
use crate::ext4::superblock::{
    Checksum, CreatorOs, ErrorPolicy, ReadOnlyCompatibleFeatures, Superblock,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const EXT2: &[u8] = include_bytes!("test-data/ext2");
const EXT4_1: &[u8] = include_bytes!("test-data/ext4-1");
const EXT4_2: &[u8] = include_bytes!("test-data/ext4-2");
const EXT4_3: &[u8] = include_bytes!("test-data/ext4-3");

#[test]
fn size_of_superblock() {
    assert_eq!(super::superblock::layout::SIZE, Some(1024));
}

#[test]
fn valid_cluster_size() {
    assert!(Superblock::new(EXT4_1).valid_cluster_size());
}

#[test]
fn valid_cluster_size_ext2() {
    assert!(Superblock::new(EXT2).valid_cluster_size());
}

#[test]
fn valid_magic() {
    assert!(Superblock::new(EXT4_1).valid_magic());
}

#[test]
fn valid_magic_ext2() {
    assert!(Superblock::new(EXT2).valid_magic());
}

#[test]
fn valid_clusters_per_group() {
    assert!(Superblock::new(EXT4_1).valid_clusters_per_group());
}

#[test]
fn valid_clusters_per_group_ext2() {
    assert!(Superblock::new(EXT2).valid_clusters_per_group());
}

#[test]
fn valid_error_policy() {
    assert!(Superblock::new(EXT4_1).valid_error_policy());
}

#[test]
fn valid_checksum() {
    assert!(Superblock::new(EXT4_1).valid_checksum());
}

#[test]
fn valid_checksum_ext2() {
    assert!(Superblock::new(EXT2).valid_checksum());
}

#[test]
fn inodes_count() {
    assert_eq!(Superblock::new(EXT4_1).inodes_count(), 64);
}

#[test]
fn inodes_count_ext2() {
    assert_eq!(Superblock::new(EXT2).inodes_count(), 64);
}

#[test]
fn blocks_count() {
    assert_eq!(Superblock::new(EXT4_1).blocks_count(), 128);
}

#[test]
fn blocks_count_ext2() {
    assert_eq!(Superblock::new(EXT2).blocks_count(), 128);
}

#[test]
fn block_size_bytes() {
    assert_eq!(Superblock::new(EXT4_1).block_size_bytes(), 4096);
}

#[test]
fn cluster_size_blocks() {
    assert_eq!(Superblock::new(EXT4_1).cluster_size_blocks(), 4096);
}

#[test]
fn mount_time_1() {
    assert_eq!(Superblock::new(EXT4_1).mount_time(), None);
}

#[test]
fn mount_time_2() {
    assert_eq!(
        Superblock::new(EXT4_2).mount_time(),
        Some(SystemTime::UNIX_EPOCH + Duration::from_secs(1732062563))
    );
}

#[test]
fn write_time_1() {
    assert_eq!(
        Superblock::new(EXT4_1).write_time(),
        SystemTime::UNIX_EPOCH + Duration::from_secs(1731941452)
    );
}

#[test]
fn write_time_2() {
    assert_eq!(
        Superblock::new(EXT4_2).write_time(),
        SystemTime::UNIX_EPOCH + Duration::from_secs(1732062580)
    );
}

#[test]
fn magic() {
    assert_eq!(Superblock::new(EXT4_1).magic(), 0xef53);
}

#[test]
fn magic_ext2() {
    assert_eq!(Superblock::new(EXT2).magic(), 0xef53);
}

#[test]
fn state() {
    assert_eq!(Superblock::new(EXT4_1).state(), State::CLEANLY_UNMOUNTED);
}

#[test]
fn state_ext2() {
    assert_eq!(Superblock::new(EXT2).state(), State::CLEANLY_UNMOUNTED);
}

#[test]
fn error_policy() {
    assert_eq!(
        Superblock::new(EXT4_1).error_policy(),
        ErrorPolicy::Continue
    );
}

#[test]
fn last_check_time() {
    assert_eq!(
        Superblock::new(EXT4_1).last_check_time(),
        SystemTime::UNIX_EPOCH + Duration::from_secs(1731941452)
    )
}

#[test]
fn check_interval_1() {
    assert_eq!(Superblock::new(EXT4_1).check_interval(), None);
}

#[test]
fn check_interval_3() {
    assert_eq!(
        Superblock::new(EXT4_3).check_interval(),
        Some(Duration::from_secs(259200))
    )
}

#[test]
fn creator_os() {
    assert_eq!(Superblock::new(EXT4_1).creator_os(), CreatorOs::Linux)
}

#[test]
fn revision_level() {
    assert_eq!(Superblock::new(EXT4_1).revision_level(), 1);
}

#[test]
fn default_reserved_user_id_1() {
    assert_eq!(Superblock::new(EXT4_1).default_reserved_user_id(), 0);
}

#[test]
fn default_reserved_user_id_3() {
    assert_eq!(Superblock::new(EXT4_3).default_reserved_user_id(), 1000);
}

#[test]
fn default_reserved_group_id_1() {
    assert_eq!(Superblock::new(EXT4_1).default_reserved_group_id(), 0);
}

#[test]
fn default_reserved_group_id_3() {
    assert_eq!(Superblock::new(EXT4_3).default_reserved_group_id(), 1000);
}

#[test]
fn first_inode() {
    assert_eq!(Superblock::new(EXT4_1).first_inode(), 11);
}

#[test]
fn inode_size() {
    assert_eq!(Superblock::new(EXT4_1).inode_size(), 256);
}

#[test]
fn block_group_number() {
    assert_eq!(Superblock::new(EXT4_1).block_group_number(), 0);
}

#[test]
fn read_only_compatible_features() {
    assert_eq!(
        Superblock::new(EXT4_1).read_only_compatible_features(),
        ReadOnlyCompatibleFeatures::SPARSE_SUPERBLOCKS
            | ReadOnlyCompatibleFeatures::CONTAINS_LARGE_FILES
            | ReadOnlyCompatibleFeatures::CONTAINS_HUGE_FILES
            | ReadOnlyCompatibleFeatures::UNLIMITED_SUBDIRECTORIES
            | ReadOnlyCompatibleFeatures::CONTAINS_LARGE_INODES
            | ReadOnlyCompatibleFeatures::METADATA_CHECKSUMS
    );
}

#[test]
fn read_only_compatible_features_ext2() {
    assert_eq!(
        Superblock::new(EXT2).read_only_compatible_features(),
        ReadOnlyCompatibleFeatures::SPARSE_SUPERBLOCKS
            | ReadOnlyCompatibleFeatures::CONTAINS_LARGE_FILES
    );
}

#[test]
fn checksum() {
    assert_eq!(
        Superblock::new(EXT4_1).checksum(),
        Checksum::Crc32c(0x42350b17)
    )
}

#[test]
fn checksum_ext2() {
    assert_eq!(Superblock::new(EXT2).checksum(), Checksum::None)
}

#[test]
fn expected_checksum() {
    assert_eq!(Superblock::new(EXT4_1).expected_checksum(), 0x42350b17)
}
