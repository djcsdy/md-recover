use binary_layout::prelude::*;

use device_descriptor::{DeviceDescriptor, NestedDeviceDescriptor};

use crate::md::superblock::Superblock;

mod device_descriptor;

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
    reserved_1: [u8; 24],
    layout: u32,
    chunk_size: u32,
    root_pv: u32,
    root_block: u32,
    reserved_2: [u8; 240],
    disks: [u8; DeviceDescriptor::<&[u8]>::SIZE * 27],
    reserved_3: [u8; 128],
    this_disk: NestedDeviceDescriptor
});

pub struct SuperblockVersion0Le<S: AsRef<[u8]>>(layout::View<S>);

impl<S: AsRef<[u8]>> SuperblockVersion0Le<S> {
    fn valid_magic(&self) -> bool {
        self.0.magic().read() == 0xa92b4efc
    }

    fn valid_major_version(&self) -> bool {
        self.0.major_version().read() == 0
    }
}

impl <S: AsRef<[u8]>> Superblock for SuperblockVersion0Le<S> {
    fn valid(&self) -> bool {
        self.valid_magic() && self.valid_major_version()
    }

    fn major_version(&self) -> u32 {
        self.0.major_version().read()
    }
}