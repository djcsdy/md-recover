use crate::ext::WideUnsigned;
use crate::ext4::crc::ext4_crc32c;
use crate::ext4::inode::flags::Flags;
use crate::ext4::inode::linux_1::NestedLinuxSpecific1;
use crate::ext4::inode::linux_2::NestedLinuxSpecific2;
use crate::ext4::inode::time::decode_extra_time;
use crate::ext4::inode::{FileMode, FileType, Permissions};
use crate::ext4::superblock::{ReadOnlyCompatibleFeatures, Superblock};
use crate::ext4::units::{BlockCount, InodeNumber};
use binary_layout::{binary_layout, Field};
use chrono::{DateTime, Duration, Utc};
use std::mem::size_of;

const BLOCKS_SIZE: usize = size_of::<u32>() * 15;

binary_layout!(layout, LittleEndian, {
    file_mode: FileMode as u16,
    user_id_low: u16,
    size_low: u32,
    access_time: u32,
    change_time: u32,
    modified_time: u32,
    delete_time: u32,
    group_id_low: u16,
    links_count: u16,
    block_count_low: u32,
    flags: Flags as u32,
    os_dependent_1: NestedLinuxSpecific1,
    blocks: [u8; BLOCKS_SIZE],
    generation: u32,
    file_acl_low: u32,
    size_high: u32,
    obsolete_fragment_address: u32,
    os_dependent_2: NestedLinuxSpecific2,
    extra_isize: u16,
    checksum_high: u16,
    change_time_extra: u32,
    modified_time_extra: u32,
    access_time_extra: u32,
    creation_time: u32,
    creation_time_extra: u32,
    version_high: u32,
    project_id: u32
});

pub struct Inode {
    storage: [u8; layout::SIZE.unwrap()],
    metadata_checksum_seed: Option<u32>,
}

impl Inode {
    pub fn new(
        superblock: &Superblock<impl AsRef<[u8]>>,
        inode_number: InodeNumber,
        buffer: impl AsRef<[u8]>,
    ) -> Self {
        let storage = {
            let source_length = layout::extra_isize::OFFSET
                + (if buffer.as_ref().len()
                    > layout::extra_isize::OFFSET + layout::extra_isize::SIZE.unwrap()
                {
                    usize::from(layout::View::new(buffer.as_ref()).extra_isize().read())
                } else {
                    0
                })
                .clamp(0, layout::SIZE.unwrap());

            let mut storage = [0u8; layout::SIZE.unwrap()];
            storage[..source_length].copy_from_slice(&buffer.as_ref()[..source_length]);
            storage
        };

        let checksum_seed = if superblock
            .read_only_compatible_features()
            .contains(ReadOnlyCompatibleFeatures::METADATA_CHECKSUMS)
        {
            Some(ext4_crc32c(
                ext4_crc32c(superblock.checksum_seed(), &inode_number.to_le_bytes()),
                &buffer.as_ref()[layout::generation::OFFSET..][..layout::generation::SIZE.unwrap()],
            ))
        } else {
            None
        };

        Self {
            storage,
            metadata_checksum_seed: checksum_seed,
        }
    }

    pub fn file_mode(&self) -> FileMode {
        self.view().file_mode().read()
    }

    pub fn file_type(&self) -> FileType {
        self.file_mode().file_type()
    }

    pub fn permissions(&self) -> Permissions {
        self.file_mode().permissions()
    }

    pub fn owner_user_id(&self) -> u32 {
        u32::from_low_high(
            self.view().user_id_low().read(),
            self.view().os_dependent_2().user_id_high().read(),
        )
    }

    pub fn file_size_bytes(&self) -> u64 {
        u64::from_low_high(
            self.view().size_low().read(),
            self.view().size_high().read(),
        )
    }

    pub fn access_time(&self) -> DateTime<Utc> {
        decode_extra_time(
            self.view().access_time().read(),
            self.view().access_time_extra().read(),
        )
    }

    pub fn change_time(&self) -> DateTime<Utc> {
        decode_extra_time(
            self.view().change_time().read(),
            self.view().change_time_extra().read(),
        )
    }

    pub fn modified_time(&self) -> DateTime<Utc> {
        decode_extra_time(
            self.view().modified_time().read(),
            self.view().modified_time_extra().read(),
        )
    }

    pub fn delete_time(&self) -> DateTime<Utc> {
        DateTime::UNIX_EPOCH + Duration::seconds(i64::from(self.view().delete_time().read()))
    }

    pub fn group_id(&self) -> u32 {
        u32::from_low_high(
            self.view().group_id_low().read(),
            self.view().os_dependent_2().group_id_high().read(),
        )
    }

    pub fn links_count(&self) -> u16 {
        self.view().links_count().read()
    }

    pub fn block_count(&self) -> BlockCount<u64> {
        BlockCount(u64::from_low_high(
            self.view().block_count_low().read(),
            u32::from(self.view().os_dependent_2().block_count_high().read()),
        ))
    }

    pub fn flags(&self) -> Flags {
        self.view().flags().read()
    }

    pub fn version(&self) -> u64 {
        u64::from_low_high(
            self.view().os_dependent_1().version().read(),
            self.view().version_high().read(),
        )
    }

    pub(in crate::ext4) fn blocks_buffer(&self) -> &[u8; BLOCKS_SIZE] {
        array_ref![self.storage.as_ref(), layout::blocks::OFFSET, BLOCKS_SIZE]
    }

    pub fn generation(&self) -> u32 {
        self.view().generation().read()
    }

    pub fn file_acl(&self) -> u64 {
        u64::from_low_high(
            self.view().file_acl_low().read(),
            u32::from(self.view().os_dependent_2().file_acl_high().read()),
        )
    }

    pub fn checksum(&self) -> u32 {
        u32::from_low_high(
            self.view().os_dependent_2().checksum_low().read(),
            self.view().checksum_high().read(),
        )
    }

    pub fn creation_time(&self) -> DateTime<Utc> {
        decode_extra_time(
            self.view().creation_time().read(),
            self.view().creation_time_extra().read(),
        )
    }

    pub fn project_id(&self) -> u32 {
        self.view().project_id().read()
    }

    pub(in crate::ext4) fn metadata_checksum_seed(&self) -> Option<u32> {
        self.metadata_checksum_seed
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.storage.as_ref())
    }
}
