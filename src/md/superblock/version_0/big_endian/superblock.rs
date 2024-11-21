use super::device_descriptor::DeviceDescriptorBigEndian;
use super::reshape_status::NestedReshapeStatusVersion0;
use crate::md::superblock::SuperblockVersion0;
use binary_layout::binary_layout;

pub use layout::View;

binary_layout!(layout, BigEndian, {
    magic: u32,
    major_version: u32,
    minor_version: u32,
    patch_version: u32,
    gvalid_words: u32,
    array_uuid_0: u32,
    ctime: u32,
    level: u32,
    size: u32,
    nr_disks: u32,
    raid_disks: u32,
    md_minor: u32,
    not_persistent: u32,
    array_uuid_1: u32,
    array_uuid_2: u32,
    array_uuid_3: u32,
    reserved_0: [u8; 64],
    utime: u32,
    state: u32,
    active_disks: u32,
    working_disks: u32,
    failed_disks: u32,
    spare_disks: u32,
    superblock_checksum: u32,
    event_count: u64,
    checkpoint_event_count: u64,
    recovery_checkpoint: u32,
    reshape_status: NestedReshapeStatusVersion0,
    reserved_1: [u8; 56],
    layout: u32,
    chunk_size: u32,
    root_pv: u32,
    root_block: u32,
    reserved_2: [u8; 240],
    disks: [u8; DeviceDescriptorBigEndian::<&[u8]>::SIZE * SuperblockVersion0::<&[u8]>::MAX_DEVICES],
    reserved_3: [u8; 128]
});

pub const SIZE: usize = match layout::SIZE {
    Some(size) => size,
    None => panic!(),
};
