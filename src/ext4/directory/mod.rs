mod dir_entry;
mod dir_entry_tail;

const NAME_LENGTH: usize = 255;

#[allow(unused_imports)]
pub use self::{dir_entry::DirEntry, dir_entry_tail::DirEntryTail};
