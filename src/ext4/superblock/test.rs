use crate::ext4::superblock::state::State;
use crate::ext4::superblock::{Checksum, ErrorPolicy, ReadOnlyCompatibleFeatures, Superblock};

const EXT2: &[u8] = include_bytes!("test-data/ext2");
const EXT4: &[u8] = include_bytes!("test-data/ext4");

#[test]
fn size_of_superblock() {
    assert_eq!(super::superblock::layout::SIZE, Some(1024));
}

#[test]
fn valid_cluster_size() {
    assert!(Superblock::new(EXT4).valid_cluster_size());
}

#[test]
fn valid_cluster_size_ext2() {
    assert!(Superblock::new(EXT2).valid_cluster_size());
}

#[test]
fn valid_magic() {
    assert!(Superblock::new(EXT4).valid_magic());
}

#[test]
fn valid_magic_ext2() {
    assert!(Superblock::new(EXT2).valid_magic());
}

#[test]
fn valid_clusters_per_group() {
    assert!(Superblock::new(EXT4).valid_clusters_per_group());
}

#[test]
fn valid_clusters_per_group_ext2() {
    assert!(Superblock::new(EXT2).valid_clusters_per_group());
}

#[test]
fn valid_checksum() {
    assert!(Superblock::new(EXT4).valid_checksum());
}

#[test]
fn valid_checksum_ext2() {
    assert!(Superblock::new(EXT2).valid_checksum());
}

#[test]
fn inodes_count() {
    assert_eq!(Superblock::new(EXT4).inodes_count(), 64);
}

#[test]
fn inodes_count_ext2() {
    assert_eq!(Superblock::new(EXT2).inodes_count(), 64);
}

#[test]
fn blocks_count() {
    assert_eq!(Superblock::new(EXT4).blocks_count(), 128);
}

#[test]
fn blocks_count_ext2() {
    assert_eq!(Superblock::new(EXT2).blocks_count(), 128);
}

#[test]
fn block_size_bytes() {
    assert_eq!(Superblock::new(EXT4).block_size_bytes(), 4096);
}

#[test]
fn cluster_size_blocks() {
    assert_eq!(Superblock::new(EXT4).cluster_size_blocks(), 4096);
}

#[test]
fn magic() {
    assert_eq!(Superblock::new(EXT4).magic(), 0xef53);
}

#[test]
fn magic_ext2() {
    assert_eq!(Superblock::new(EXT2).magic(), 0xef53);
}

#[test]
fn state() {
    assert_eq!(Superblock::new(EXT4).state(), State::CLEANLY_UNMOUNTED);
}

#[test]
fn state_ext2() {
    assert_eq!(Superblock::new(EXT2).state(), State::CLEANLY_UNMOUNTED);
}

#[test]
fn error_policy() {
    assert_eq!(Superblock::new(EXT4).error_policy(), ErrorPolicy::Continue);
}

#[test]
fn read_only_compatible_features() {
    assert_eq!(
        Superblock::new(EXT4).read_only_compatible_features(),
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
        Superblock::new(EXT4).checksum(),
        Checksum::Crc32c(0x42350b17)
    )
}

#[test]
fn checksum_ext2() {
    assert_eq!(Superblock::new(EXT2).checksum(), Checksum::None)
}

#[test]
fn expected_checksum() {
    assert_eq!(Superblock::new(EXT4).expected_checksum(), 0x42350b17)
}
