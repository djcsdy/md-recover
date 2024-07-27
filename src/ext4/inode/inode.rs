use binary_layout::define_layout;
use std::mem::size_of;

const NUM_BLOCKS: usize = 15;

define_layout!(layout, LittleEndian, {
    file_mode: u16,
    user_id_low: u16,
    size_low: u32,
    access_time: u32,
    change_time: u32,
    modified_time: u32,
    delete_time: u32,
    group_id_low: u16,
    links_count: u16,
    block_count_low: u16,
    flags: u32,
    os_dependent_1: u32,
    blocks: [u8; size_of::<u32>() * NUM_BLOCKS],
    generation: u32,
    file_acl_low: u32,
    size_high: u32,
    obsolete_fragment_address: u32,
    os_dependent_2: [u8; 12],
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

pub struct Inode<S: AsRef<[u8]>>(layout::View<S>);
