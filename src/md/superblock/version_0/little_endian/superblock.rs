use super::device_descriptor;
use binary_layout::define_layout;

pub use self::layout::View;

define_layout!(layout, LittleEndian, {
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
    failed_disks: u32,spare_disks: u32,
    superblock_checksum: u32,
    event_count: u64,
    checkpoint_event_count: u64,
    recovery_checkpoint: u32,
    reshape_position: u64,
    new_level: u32,
    delta_disks: u32,
    new_layout: u32,
    new_chunk: u32,
    reserved_1: [u8; 56],
    layout: u32,
    chunk_size: u32,
    root_pv: u32,
    root_block: u32,
    reserved_2: [u8; 240],
    disks: [u8; device_descriptor::SIZE * 27],
    reserved_3: [u8; 128]
});

pub const SIZE: usize = match layout::SIZE {
    Some(size) => size,
    None => panic!(),
};
