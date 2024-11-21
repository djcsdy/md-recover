use super::device_descriptor::DeviceDescriptorBigEndian;
use super::reshape_status::NestedReshapeStatusVersion0;
use crate::md::superblock::version_0::device_descriptor::DeviceDescriptor;
use crate::md::superblock::version_0::MAX_DEVICES;
use crate::md::superblock::{ReshapeStatus, SuperblockVersion0};
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
    disks: [u8; DeviceDescriptorBigEndian::<&[u8]>::SIZE * MAX_DEVICES],
    reserved_3: [u8; 128]
});

pub const SIZE: usize = match layout::SIZE {
    Some(size) => size,
    None => panic!(),
};

impl<S: AsRef<[u8]>> SuperblockVersion0 for View<S> {
    fn valid_device_descriptors(&self) -> bool {
        let buffer = self.disks();
        (0..MAX_DEVICES)
            .map(|i| {
                DeviceDescriptorBigEndian::new(array_ref![
                    buffer,
                    i * DeviceDescriptorBigEndian::<&[u8]>::SIZE,
                    DeviceDescriptorBigEndian::<&[u8]>::SIZE
                ])
            })
            .all(|descriptor| descriptor.is_valid())
    }

    fn magic(&self) -> u32 {
        self.magic().read()
    }

    fn major_version(&self) -> u32 {
        self.major_version().read()
    }

    fn minor_version(&self) -> u32 {
        self.minor_version().read()
    }

    fn array_uuid_0(&self) -> u32 {
        self.array_uuid_0().read()
    }

    fn level(&self) -> u32 {
        self.level().read()
    }

    fn size(&self) -> u32 {
        self.size().read()
    }

    fn raid_disks(&self) -> u32 {
        self.raid_disks().read()
    }

    fn array_uuid_1(&self) -> u32 {
        self.array_uuid_1().read()
    }

    fn array_uuid_2(&self) -> u32 {
        self.array_uuid_2().read()
    }

    fn array_uuid_3(&self) -> u32 {
        self.array_uuid_3().read()
    }

    fn event_count(&self) -> u64 {
        self.event_count().read()
    }

    fn reshape_status(&self) -> ReshapeStatus {
        self.reshape_status().into()
    }

    fn layout(&self) -> u32 {
        self.layout().read()
    }

    fn chunk_size(&self) -> u32 {
        self.chunk_size().read()
    }

    fn device_roles(&self) -> Vec<u16> {
        let buffer = self.disks();
        Vec::from_iter((0..MAX_DEVICES).map(|i| {
            DeviceDescriptorBigEndian::new(array_ref![
                buffer,
                i * DeviceDescriptorBigEndian::<&[u8]>::SIZE,
                DeviceDescriptorBigEndian::<&[u8]>::SIZE
            ])
            .role()
            .try_into()
            .unwrap()
        }))
    }
}
