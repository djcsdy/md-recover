use crate::ext4::block_group::{BlockGroupDescriptor, Flags};
use crate::ext4::units::FsBlockIndex;

const DESCRIPTOR: &[u8] = include_bytes!("test_data/descriptor");

#[test]
fn parse() {
    let descriptor = BlockGroupDescriptor::new(DESCRIPTOR);
    assert_eq!(descriptor.block_bitmap_block(), FsBlockIndex(0xe));
    assert_eq!(descriptor.inode_bitmap_block(), FsBlockIndex(0x1e));
    assert_eq!(descriptor.inode_table_block(), FsBlockIndex(0x2e));
    assert_eq!(descriptor.free_block_count(), 22954);
    assert_eq!(descriptor.free_inode_count(), 25589);
    assert_eq!(descriptor.used_directories_count(), 2);
    assert_eq!(descriptor.flags(), Flags::INODE_TABLE_ZEROED);
    assert_eq!(descriptor.exclude_bitmap_block(), FsBlockIndex(0));
    assert_eq!(descriptor.block_bitmap_checksum(), 0x30609723);
    assert_eq!(descriptor.inode_bitmap_checksum(), 0x957502e9);
    assert_eq!(descriptor.unused_inode_count(), 0x63f5);
    assert_eq!(descriptor.checksum(), 0x67e2);
}
