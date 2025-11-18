use crate::ext4::crc::EXT4_CRC32C;
use crate::ext4::extent::header::NestedExtentHeader;
use crate::ext4::extent::tail;
use crate::ext4::extent::{header, index};
use crate::ext4::inode::Inode;
use binary_layout::{binary_layout, Field};
use crc::Crc;

binary_layout!(layout, LittleEndian, {
    header: NestedExtentHeader,
    indices_and_tail: [u8]
});

pub struct ExtentTree<S: AsRef<[u8]>> {
    storage: S,
    checksum_seed: Option<u32>,
}

impl<'inode> ExtentTree<&'inode [u8]> {
    pub fn from_inode(inode: &'inode Inode) -> Self {
        Self {
            storage: inode.blocks_buffer(),
            checksum_seed: None,
        }
    }
}

impl<S: AsRef<[u8]>> ExtentTree<S> {
    const MAGIC: u16 = 0xf30a;

    pub fn from_block(inode: &Inode, block: S) -> Self {
        Self {
            storage: block,
            checksum_seed: inode.metadata_checksum_seed(),
        }
    }

    pub fn valid(&self) -> bool {
        if self.storage.as_ref().len() < header::layout::SIZE.unwrap() {
            return false;
        }

        if self.view().header().magic().read() != Self::MAGIC {
            return false;
        }

        let indices_size =
            usize::from(self.view().header().entries().read()) * index::layout::SIZE.unwrap();

        let tail_size = match self.checksum_seed {
            Some(_) => tail::layout::SIZE.unwrap(),
            None => 0,
        };

        if self.view().indices_and_tail().as_ref().len() < indices_size + tail_size {
            return false;
        }

        match self.checksum_seed {
            None => true,
            Some(checksum_seed) => {
                let expected_checksum = {
                    let tail_offset = layout::indices_and_tail::OFFSET + indices_size;

                    let crc = Crc::<u32>::new(&EXT4_CRC32C);
                    let mut digest = crc.digest_with_initial(checksum_seed);
                    digest.update(&self.storage.as_ref()[..tail_offset]);
                    digest.finalize()
                };

                tail::layout::View::new(&self.view().indices_and_tail()[indices_size..])
                    .checksum()
                    .read()
                    == expected_checksum
            }
        }
    }

    pub fn max(&self) -> u16 {
        self.view().header().max().read()
    }

    pub fn depth(&self) -> u16 {
        self.view().header().depth().read()
    }

    pub fn generation(&self) -> u32 {
        self.view().header().generation().read()
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.storage.as_ref())
    }
}

#[cfg(test)]
mod test {
    use crate::ext4::extent::tree::ExtentTree;
    use crate::ext4::inode::Inode;
    use crate::ext4::superblock::Superblock;

    const SUPERBLOCK: &[u8] = include_bytes!("test_data/superblock");
    const ROOT_INODE: &[u8] = include_bytes!("test_data/root_inode");

    #[test]
    fn test() -> anyhow::Result<()> {
        let superblock = Superblock::new(SUPERBLOCK);
        let inode = Inode::new(&superblock, 2, ROOT_INODE);
        let tree = ExtentTree::from_inode(&inode);
        assert!(tree.valid());
        assert_eq!(tree.max(), 4);
        assert_eq!(tree.depth(), 0);
        assert_eq!(tree.generation(), 0);
        Ok(())
    }
}
