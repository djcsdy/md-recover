use binary_layout::binary_layout;

use super::NAME_LENGTH;

binary_layout!(layout, LittleEndian, {
    inode: u32,
    record_length: u16,
    name_length: u16,
    name: [u8; NAME_LENGTH]
});

pub struct DirEntry1<S: AsRef<[u8]>>(layout::View<S>);
