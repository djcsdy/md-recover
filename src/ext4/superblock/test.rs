use crate::ext4::superblock::Superblock;

const VALID_1: &[u8] = include_bytes!("./test-data/valid-1");

#[test]
fn size_of_superblock() {
    assert_eq!(super::superblock::layout::SIZE, Some(1024));
}

#[test]
fn inodes_count() {
    assert_eq!(Superblock::new(VALID_1).inodes_count(), 64);
}
