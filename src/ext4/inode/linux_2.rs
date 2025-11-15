use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    block_count_high: u16,
    file_acl_high: u16,
    user_id_high: u16,
    group_id_high: u16,
    checksum_low: u16,
    reserved: u16,
});

pub use layout::NestedView as NestedLinuxSpecific2;
