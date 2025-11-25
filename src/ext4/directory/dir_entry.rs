use binary_layout::binary_layout;
use file_type::Ext4DirectoryInlineFileType;

use super::{file_type, NAME_LENGTH};

binary_layout!(layout, LittleEndian, {
    inode: u32,
    record_length: u16,
    name_length: u8,
    file_type: Ext4DirectoryInlineFileType as u8,
    name: [u8; NAME_LENGTH]
});

pub struct DirEntry<S: AsRef<[u8]>>(layout::View<S>);
