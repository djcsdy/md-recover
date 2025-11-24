use binary_layout::binary_layout;

use super::NAME_LENGTH;

binary_layout!(layout, LittleEndian, {
    inode: u32,
    record_length: u16,
    name_length: u8,
    file_type: u8,
    name: [u8; NAME_LENGTH]
});

pub struct DirEntry<S: AsRef<[u8]>>(layout::View<S>);
