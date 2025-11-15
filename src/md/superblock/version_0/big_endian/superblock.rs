use super::device_descriptor::DeviceDescriptorBigEndian;
use super::reshape_status::NestedReshapeStatusVersion0;
use crate::md::superblock::SuperblockVersion0;
use binary_layout::binary_layout;

#[allow(unused_imports)]
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
    disks: [u8; DeviceDescriptorBigEndian::<&[u8]>::SIZE * SuperblockVersion0::MAX_DEVICES],
    reserved_3: [u8; 128]
});

pub const SIZE: usize = match layout::SIZE {
    Some(size) => size,
    None => panic!(),
};

impl<S: AsRef<[u8]>> From<View<S>> for SuperblockVersion0 {
    fn from(value: View<S>) -> Self {
        let device_descriptor_buffer = value.disks().as_slice();
        let disks = Vec::from_iter((0..SuperblockVersion0::MAX_DEVICES).map(|i| {
            DeviceDescriptorBigEndian::new(array_ref![
                device_descriptor_buffer,
                i * DeviceDescriptorBigEndian::<&[u8]>::SIZE,
                DeviceDescriptorBigEndian::<&[u8]>::SIZE
            ])
            .into()
        }));

        Self {
            magic: value.magic().read(),
            major_version: value.major_version().read(),
            minor_version: value.minor_version().read(),
            patch_version: value.patch_version().read(),
            gvalid_words: value.gvalid_words().read(),
            array_uuid_0: value.array_uuid_0().read(),
            ctime: value.ctime().read(),
            level: value.level().read(),
            size: value.size().read(),
            nr_disks: value.nr_disks().read(),
            raid_disks: value.raid_disks().read(),
            md_minor: value.md_minor().read(),
            not_persistent: value.not_persistent().read(),
            array_uuid_1: value.array_uuid_1().read(),
            array_uuid_2: value.array_uuid_2().read(),
            array_uuid_3: value.array_uuid_3().read(),
            utime: value.utime().read(),
            state: value.state().read(),
            active_disks: value.active_disks().read(),
            working_disks: value.working_disks().read(),
            failed_disks: value.failed_disks().read(),
            spare_disks: value.spare_disks().read(),
            superblock_checksum: value.superblock_checksum().read(),
            event_count: value.event_count().read(),
            checkpoint_event_count: value.checkpoint_event_count().read(),
            recovery_checkpoint: value.recovery_checkpoint().read(),
            reshape_status: value.reshape_status().into(),
            layout: value.layout().read(),
            chunk_size: value.chunk_size().read(),
            root_pv: value.root_pv().read(),
            root_block: value.root_block().read(),
            disks,
        }
    }
}
