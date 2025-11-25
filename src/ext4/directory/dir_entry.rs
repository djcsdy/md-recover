use super::file_type;
use crate::ext4::string::Ext4String;
use crate::ext4::units::InodeNumber;
use binary_layout::{binary_layout, Field};
use file_type::Ext4DirectoryInlineFileType;

binary_layout!(layout, LittleEndian, {
    inode: InodeNumber as u32,
    record_length: u16,
    name_length: u8,
    file_type: Ext4DirectoryInlineFileType as u8,
    name: [u8]
});

pub struct DirEntry<S: AsRef<[u8]>>(S);

impl<'buf> DirEntry<&'buf [u8]> {
    pub(super) fn from_buf(buf: &'buf [u8]) -> (Option<Self>, &'buf [u8]) {
        if buf.len() >= layout::name::OFFSET {
            let view = layout::View::new(buf);
            let record_length = usize::from(view.record_length().read());
            let name_length = usize::from(view.name_length().read());
            if record_length <= buf.len() && record_length >= (layout::name::OFFSET + name_length) {
                return (Some(Self(&buf[..record_length])), &buf[record_length..]);
            }
        }

        (None, buf)
    }
}

impl<S: AsRef<[u8]>> DirEntry<S> {
    pub fn name(&self) -> Ext4String<&[u8]> {
        Ext4String::from_null_terminated_bytes(self.view().into_name().into_slice())
    }

    pub fn file_type(&self) -> Ext4DirectoryInlineFileType {
        self.view().file_type().read()
    }

    pub fn inode(&self) -> InodeNumber {
        self.view().inode().read()
    }

    pub fn to_owned(&self) -> DirEntry<Vec<u8>> {
        DirEntry(self.0.as_ref().to_owned())
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.0.as_ref())
    }
}

#[cfg(test)]
mod test {
    use crate::ext4::directory::dir_entry::DirEntry;
    use crate::ext4::directory::file_type::Ext4DirectoryInlineFileType;
    use crate::ext4::string::Ext4String;
    use crate::ext4::units::InodeNumber;

    static DIR_ENTRY: &[u8] = include_bytes!("test_data/dir_entry");
    static DIR_ENTRIES_WITH_PADDING: &[u8] = include_bytes!("test_data/dir_entries_with_padding");

    #[test]
    fn dir_entry() {
        let (Some(dir_entry), rest) = DirEntry::from_buf(DIR_ENTRY) else {
            panic!("Expected DirEntry")
        };

        assert_eq!(dir_entry.name(), Ext4String::from("."));
        assert_eq!(
            dir_entry.file_type(),
            Ext4DirectoryInlineFileType::Directory
        );
        assert_eq!(dir_entry.inode(), InodeNumber(2));

        assert_eq!(rest, &[]);
    }

    #[test]
    fn dir_entries_with_padding() {
        let (Some(dir_entry), rest) = DirEntry::from_buf(DIR_ENTRIES_WITH_PADDING) else {
            panic!("Expected DirEntry")
        };

        assert_eq!(dir_entry.name(), Ext4String::from("."));
        assert_eq!(
            dir_entry.file_type(),
            Ext4DirectoryInlineFileType::Directory
        );
        assert_eq!(dir_entry.inode(), InodeNumber(2));

        let (Some(dir_entry), rest) = DirEntry::from_buf(rest) else {
            panic!("Expected DirEntry")
        };

        assert_eq!(dir_entry.name(), Ext4String::from(".."));
        assert_eq!(
            dir_entry.file_type(),
            Ext4DirectoryInlineFileType::Directory
        );
        assert_eq!(dir_entry.inode(), InodeNumber(2));

        let (None, rest) = DirEntry::from_buf(rest) else {
            panic!("Expected None")
        };

        assert_eq!(rest, &[0; 96]);
    }
}
