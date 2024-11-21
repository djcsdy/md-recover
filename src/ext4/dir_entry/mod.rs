mod dir_entry_1;
mod dir_entry_2;
mod dir_entry_tail;

const NAME_LENGTH: usize = 255;

pub use dir_entry_1::DirEntry1;
pub use dir_entry_2::DirEntry2;
pub use dir_entry_tail::DirEntryTail;
