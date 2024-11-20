use super::checksum_type::ChecksumType;
use super::flags::Flags;
use super::{
    CompatibleFeatures, CreatorOs, ErrorPolicy, IncompatibleFeatures, ReadOnlyCompatibleFeatures,
    State,
};
use crate::ext4::superblock::checksum::Checksum;
use binary_layout::prelude::*;
use crc::{Algorithm, Crc, CRC_32_ISCSI};
use std::io::{Error, ErrorKind, Read, Result};
use std::time::{Duration, SystemTime};

binary_layout!(layout, LittleEndian, {
    inodes_count: u32,
    blocks_count_low: u32,
    reserved_blocks_count_low: u32,
    free_blocks_count_low: u32,
    free_inodes_count: u32,
    first_data_block: u32,
    log_block_size: u32,
    log_cluster_size: u32,
    blocks_per_group: u32,
    clusters_per_group: u32,
    inodes_per_group: u32,
    mount_time_low: u32,
    write_time_low: u32,
    mount_count: u16,
    max_mount_count: u16,
    magic: u16,
    state: State as u16,
    error_policy: ErrorPolicy as u16,
    minor_revision_level: u16,
    last_check_time_low: u32,
    check_interval: u32,
    creator_os: CreatorOs as u32,
    revision_level: u32,
    default_reserved_user_id: u16,
    default_reserved_group_id: u16,
    first_inode: u32,
    inode_size: u16,
    block_group_number: u16,
    compatible_features: CompatibleFeatures as u32,
    incompatible_features: IncompatibleFeatures as u32,
    read_only_compatible_features: ReadOnlyCompatibleFeatures as u32,
    uuid: [u8; 16],
    volume_name: [u8; 16],
    last_mounted: [u8; 64],
    algorithm_usage_bitmap: u32,
    preallocate_blocks: u8,
    preallocate_directory_blocks: u8,
    reserved_gdt_blocks: u16,
    journal_uuid: [u8; 16],
    journal_inode_number: u32,
    journal_device_number: u32,
    last_orphan: u32,
    hash_seed: [u8; 16],
    default_hash_version: u8,
    journal_backup_type: u8,
    group_descriptor_size: u16,
    default_mount_options: u32,
    first_meta_block_group: u32,
    mkfs_time_low: u32,
    journal_blocks: [u8; 68],
    blocks_count_high: u32,
    reserved_blocks_count_high: u32,
    free_blocks_count_high: u32,
    minimum_extra_inode_size: u16,
    wanted_extra_inode_size: u16,
    flags: Flags as u32,
    raid_stride: u16,
    multi_mount_prevention_interval: u16,
    multi_mount_prevention_block: u64,
    raid_stripe_width: u32,
    log_groups_per_flex: u8,
    checksum_type: ChecksumType as u8,
    reserved_0: u16,
    kbytes_written: u64,
    snapshot_inode_number: u32,
    snapshot_id: u32,
    snapshot_reserved_blocks_count: u64,
    snapshot_list: u32,
    error_count: u32,
    first_error_time_low: u32,
    first_error_inode: u32,
    first_error_block: u64,
    first_error_function: [u8; 32],
    first_error_line: u32,
    last_error_time_low: u32,
    last_error_inode: u32,
    last_error_line: u32,
    last_error_block: u64,
    last_error_function: [u8; 32],
    mount_options: [u8; 64],
    user_quota_inode_number: u32,
    group_quota_inode_number: u32,
    overhead_blocks: u32,
    backup_block_group_0: u32,
    backup_block_group_1: u32,
    encryption_algorithms: [u8; 4],
    encryption_password_salt: [u8; 16],
    lost_and_found_inode_number: u32,
    project_quota_inode_number: u32,
    checksum_seed: u32,
    write_time_high: u8,
    mount_time_high: u8,
    mkfs_time_high: u8,
    last_check_time_high: u8,
    first_error_time_high: u8,
    last_error_time_high: u8,
    reserved_1: [u8; 2],
    filename_character_encoding: u16,
    filename_character_encoding_flags: u16,
    orphan_file_inode_number: u32,
    reserved_2: [u8; 376],
    checksum: u32
});

pub struct Superblock<S: AsRef<[u8]>>(S);

impl<S: AsRef<[u8]>> Superblock<S> {
    const EXT4_CRC32C: Algorithm<u32> = Algorithm {
        width: CRC_32_ISCSI.width,
        poly: CRC_32_ISCSI.poly,
        init: CRC_32_ISCSI.init,
        refin: CRC_32_ISCSI.refin,
        refout: CRC_32_ISCSI.refout,
        xorout: 0,
        check: CRC_32_ISCSI.check,
        residue: CRC_32_ISCSI.residue,
    };

    pub fn new(storage: S) -> Self {
        Self(storage)
    }

    pub fn valid(&self) -> bool {
        self.valid_cluster_size()
            && self.valid_clusters_per_group()
            && self.valid_magic()
            && self.valid_error_policy()
            && self.valid_checksum()
    }

    pub fn valid_cluster_size(&self) -> bool {
        self.read_only_compatible_features()
            .contains(ReadOnlyCompatibleFeatures::BIGALLOC)
            || self.cluster_size_blocks() == self.block_size_bytes()
    }

    pub fn valid_clusters_per_group(&self) -> bool {
        self.read_only_compatible_features()
            .contains(ReadOnlyCompatibleFeatures::BIGALLOC)
            || self.clusters_per_group() == self.blocks_per_group()
    }

    pub fn valid_magic(&self) -> bool {
        self.magic() == 0xef53
    }

    pub fn valid_error_policy(&self) -> bool {
        match self.error_policy() {
            ErrorPolicy::Unknown(_) => false,
            _ => true,
        }
    }

    pub fn valid_checksum(&self) -> bool {
        match self.checksum() {
            Checksum::None => true,
            Checksum::Crc32c(checksum) => checksum == self.expected_checksum(),
            Checksum::Unknown(_, _) => false,
        }
    }

    pub fn inodes_count(&self) -> u32 {
        self.view().into_inodes_count().read()
    }

    pub fn blocks_count(&self) -> u64 {
        let view = self.view();
        view.blocks_count_low().read() as u64 | ((view.blocks_count_high().read() as u64) << 32)
    }

    pub fn reserved_blocks_count(&self) -> u64 {
        let view = self.view();
        view.reserved_blocks_count_low().read() as u64
            | ((view.reserved_blocks_count_high().read() as u64) << 32)
    }

    pub fn free_blocks_count(&self) -> u64 {
        let view = self.view();
        view.free_blocks_count_low().read() as u64
            | ((view.free_blocks_count_high().read() as u64) << 32)
    }

    pub fn free_inodes_count(&self) -> u32 {
        self.view().into_free_inodes_count().read()
    }

    pub fn first_data_block(&self) -> u32 {
        self.view().into_first_data_block().read()
    }

    pub fn block_size_bytes(&self) -> u64 {
        1 << (10 + self.view().into_log_block_size().read())
    }

    pub fn cluster_size_blocks(&self) -> u64 {
        1 << (10 + self.view().into_log_cluster_size().read())
    }

    pub fn blocks_per_group(&self) -> u32 {
        self.view().blocks_per_group().read()
    }

    pub fn clusters_per_group(&self) -> u32 {
        self.view().clusters_per_group().read()
    }

    pub fn inodes_per_group(&self) -> u32 {
        self.view().into_inodes_per_group().read()
    }

    pub fn mount_time(&self) -> Option<SystemTime> {
        let view = self.view();
        let seconds =
            (view.mount_time_low().read() as u64) | ((view.mount_time_high().read() as u64) << 32);
        if seconds == 0 {
            None
        } else {
            Some(SystemTime::UNIX_EPOCH + Duration::from_secs(seconds))
        }
    }

    pub fn write_time(&self) -> SystemTime {
        let view = self.view();
        SystemTime::UNIX_EPOCH
            + Duration::from_secs(
                (view.write_time_low().read() as u64)
                    | ((view.write_time_high().read() as u64) << 32),
            )
    }

    pub fn mount_count(&self) -> u16 {
        self.view().into_mount_count().read()
    }

    pub fn max_mount_count(&self) -> u16 {
        self.view().into_max_mount_count().read()
    }

    pub fn magic(&self) -> u16 {
        self.view().into_magic().read()
    }

    pub fn state(&self) -> State {
        self.view().into_state().read()
    }

    pub fn error_policy(&self) -> ErrorPolicy {
        self.view().into_error_policy().read()
    }

    pub fn minor_revision_level(&self) -> u16 {
        self.view().into_minor_revision_level().read()
    }

    pub fn last_check_time(&self) -> SystemTime {
        let view = self.view();
        SystemTime::UNIX_EPOCH
            + Duration::from_secs(
                (view.last_check_time_low().read() as u64)
                    | ((view.last_check_time_high().read() as u64) << 32),
            )
    }

    pub fn check_interval(&self) -> Option<Duration> {
        let seconds = self.view().into_check_interval().read().into();
        if seconds == 0 {
            None
        } else {
            Some(Duration::from_secs(seconds))
        }
    }

    pub fn creator_os(&self) -> CreatorOs {
        self.view().into_creator_os().read()
    }

    pub fn revision_level(&self) -> u32 {
        self.view().into_revision_level().read()
    }

    pub fn default_reserved_user_id(&self) -> u16 {
        self.view().into_default_reserved_user_id().read()
    }

    pub fn default_reserved_group_id(&self) -> u16 {
        self.view().into_default_reserved_group_id().read()
    }

    pub fn first_inode(&self) -> u32 {
        self.view().into_first_inode().read()
    }

    pub fn inode_size(&self) -> u16 {
        self.view().into_inode_size().read()
    }

    pub fn block_group_number(&self) -> u16 {
        self.view().into_block_group_number().read()
    }

    pub fn compatible_features(&self) -> CompatibleFeatures {
        self.view().into_compatible_features().read()
    }

    pub fn incompatible_features(&self) -> IncompatibleFeatures {
        self.view().into_incompatible_features().read()
    }

    pub fn read_only_compatible_features(&self) -> ReadOnlyCompatibleFeatures {
        self.view().into_read_only_compatible_features().read()
    }

    pub fn checksum(&self) -> Checksum {
        let view = self.view();
        match view.checksum_type().read() {
            ChecksumType::None => Checksum::None,
            ChecksumType::Crc32c => Checksum::Crc32c(view.checksum().read()),
            ChecksumType::Unknown(t) => Checksum::Unknown(t, view.checksum().read()),
        }
    }

    pub fn expected_checksum(&self) -> u32 {
        let crc = Crc::<u32>::new(&Self::EXT4_CRC32C);
        let mut digest = crc.digest();
        digest.update(&self.0.as_ref()[0..layout::checksum::OFFSET]);
        digest.finalize()
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.0.as_ref())
    }
}

impl Superblock<Vec<u8>> {
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        let mut buf = vec![0u8; layout::SIZE.unwrap()];
        reader.read_exact(&mut buf)?;
        let superblock = Self::new(buf);
        if superblock.valid() {
            Ok(superblock)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}
