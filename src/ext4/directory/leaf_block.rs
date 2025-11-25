use crate::ext4::crc::EXT4_CRC32C;
use crate::ext4::directory::dir_entry::DirEntry;
use crate::ext4::directory::dir_entry_tail;

pub struct Ext4DirectoryLeafBlock<S: AsRef<[u8]>> {
    storage: S,
    pos: usize,
    len: usize,
}

impl<S: AsRef<[u8]>> Ext4DirectoryLeafBlock<S> {
    pub(super) fn from_block_and_checksum_seed(
        block: S,
        checksum_seed: Option<u32>,
    ) -> Option<Self> {
        let len = if let Some(checksum_seed) = checksum_seed {
            let block = block.as_ref();
            let tail_size = dir_entry_tail::layout::SIZE.unwrap();
            let tail_offset = block.len() - tail_size;

            let tail = dir_entry_tail::layout::View::new(&block[tail_offset..]);
            if tail.reserved_zero_1().read() != 0
                || usize::from(tail.record_length().read()) != tail_size
                || tail.reserved_zero_2().read() != 0
                || tail.reserved_file_type().read() != 0xde
            {
                return None;
            }

            let expected_checksum = {
                let mut digest = EXT4_CRC32C.digest_with_initial(checksum_seed.reverse_bits());
                digest.update(&block[..tail_offset]);
                digest.finalize()
            };

            if tail.checksum().read() != expected_checksum {
                return None;
            }

            tail_offset
        } else {
            block.as_ref().len()
        };

        Some(Self {
            storage: block,
            pos: 0,
            len,
        })
    }

    pub fn read_next_entry(&mut self) -> Option<DirEntry<&[u8]>> {
        let rest = &self.storage.as_ref()[self.pos..self.len];
        let (entry, rest) = DirEntry::from_buf(rest);
        self.pos = rest.as_ptr() as usize - self.storage.as_ref().as_ptr() as usize;
        entry
    }
}

#[cfg(test)]
mod test {
    use crate::ext4::directory::file_type::Ext4DirectoryInlineFileType;
    use crate::ext4::directory::leaf_block::Ext4DirectoryLeafBlock;
    use crate::ext4::string::Ext4String;
    use crate::ext4::units::InodeNumber;

    static EMPTY_ROOT_DIR: &[u8] = include_bytes!("test_data/empty-root-dir-leaf");

    #[test]
    fn read_empty_root_dir() {
        let mut leaf =
            Ext4DirectoryLeafBlock::from_block_and_checksum_seed(EMPTY_ROOT_DIR, Some(1485601019))
                .unwrap();

        let entry = leaf.read_next_entry().unwrap();
        assert_eq!(entry.name(), Ext4String::from("."));
        assert_eq!(entry.file_type(), Ext4DirectoryInlineFileType::Directory);
        assert_eq!(entry.inode(), InodeNumber(2));

        let entry = leaf.read_next_entry().unwrap();
        assert_eq!(entry.name(), Ext4String::from(".."));
        assert_eq!(entry.file_type(), Ext4DirectoryInlineFileType::Directory);
        assert_eq!(entry.inode(), InodeNumber(2));

        let entry = leaf.read_next_entry().unwrap();
        assert_eq!(entry.name(), Ext4String::from("lost+found"));
        assert_eq!(entry.file_type(), Ext4DirectoryInlineFileType::Directory);
        assert_eq!(entry.inode(), InodeNumber(11));

        assert!(leaf.read_next_entry().is_none());
    }
}
