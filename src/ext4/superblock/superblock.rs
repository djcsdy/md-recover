use super::state::State;
use binary_layout::prelude::*;
use crc::{Crc, CRC_32_ISCSI};

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
    errors: u16,
    minor_revision_level: u16,
    last_check_time_low: u32,
    check_interval: u32,
    creator_os: u32,
    revision_level: u32,
    default_reserved_user_id: u16,
    default_reserved_group_id: u16,
    first_inode: u32,
    inode_size: u16,
    block_group_number: u16,
    compatible_features: u32,
    incompatible_features: u32,
    read_only_compatible_features: u32,
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
    flags: u32,
    raid_stride: u16,
    multi_mount_prevention_interval: u16,
    multi_mount_prevention_block: u64,
    raid_stripe_width: u32,
    log_groups_per_flex: u8,
    checksum_type: u8,
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
    pub fn new(storage: S) -> Self {
        Self(storage)
    }

    pub fn valid_checksum(&self) -> bool {
        self.compute_checksum() == layout::View::new(self.0.as_ref()).into_checksum().read()
    }

    fn compute_checksum(&self) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_ISCSI);
        let mut digest = crc.digest();
        digest.update(&self.0.as_ref()[0..layout::SIZE.unwrap() - size_of::<u32>()]);
        digest.finalize()
    }
}
