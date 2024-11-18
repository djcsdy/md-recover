use crate::ext4::superblock::Superblock;

const EXT2: &[u8] = include_bytes!("test-data/ext2");

#[test]
fn size_of_superblock() {
    assert_eq!(super::superblock::layout::SIZE, Some(1024));
}

#[test]
fn inodes_count_ext2() {
    assert_eq!(Superblock::new(EXT2).inodes_count(), 64);
}

#[test]
fn blocks_count_ext2() {
    assert_eq!(Superblock::new(EXT2).blocks_count(), 128);
}
