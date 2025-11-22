use crate::ext4::crc::EXT4_CRC32C;
use crate::ext4::extent::extent::Extent;
use crate::ext4::extent::header::NestedExtentHeader;
use crate::ext4::extent::index::ExtentIndex;
use crate::ext4::extent::{extent, tail};
use crate::ext4::extent::{header, index};
use crate::ext4::inode::Inode;
use binary_layout::{binary_layout, Field};
use crc::Crc;
use std::iter::FusedIterator;

#[derive(PartialEq, Clone, Hash)]
pub enum ExtentTree<S: AsRef<[u8]>> {
    Branch(ExtentBranch<S>),
    Leaf(ExtentLeaf<S>),
}

impl<'inode> ExtentTree<&'inode [u8]> {
    pub fn from_inode(inode: &'inode Inode) -> Self {
        Self::from_internal(ExtentTreeInternal::from_inode(inode))
    }
}

impl<S: AsRef<[u8]>> ExtentTree<S> {
    pub fn from_block(inode: &Inode, block: S) -> Self {
        Self::from_internal(ExtentTreeInternal::from_block(inode, block))
    }

    fn from_internal(internal: ExtentTreeInternal<S>) -> Self {
        if internal.depth() == 0 {
            Self::Leaf(ExtentLeaf(internal))
        } else {
            Self::Branch(ExtentBranch(internal))
        }
    }

    pub fn valid(&self) -> bool {
        match self {
            ExtentTree::Branch(branch) => branch.valid(),
            ExtentTree::Leaf(leaf) => leaf.valid(),
        }
    }

    pub fn to_owned(&self) -> ExtentTree<Vec<u8>> {
        match self {
            ExtentTree::Branch(branch) => ExtentTree::Branch(branch.to_owned()),
            ExtentTree::Leaf(leaf) => ExtentTree::Leaf(leaf.to_owned()),
        }
    }
}

#[derive(PartialEq, Clone, Hash)]
pub struct ExtentBranch<S: AsRef<[u8]>>(ExtentTreeInternal<S>);

impl<S: AsRef<[u8]>> ExtentBranch<S> {
    pub fn valid(&self) -> bool {
        self.0.valid()
    }

    pub fn subtree_count(&self) -> usize {
        self.0.entry_count()
    }

    pub fn max_subtrees(&self) -> usize {
        self.0.max_entries()
    }

    pub fn iter_subtrees(&'_ self) -> ExtentBranchIter<'_, S> {
        ExtentBranchIter {
            branch: self,
            pos: 0,
        }
    }

    fn subtrees_storage(&self) -> &[u8] {
        self.0.entries_storage()
    }

    pub fn generation(&self) -> u32 {
        self.0.generation()
    }

    pub fn to_owned(&self) -> ExtentBranch<Vec<u8>> {
        ExtentBranch(self.0.to_owned())
    }
}

#[derive(PartialEq, Clone, Hash)]
pub struct ExtentBranchIter<'branch, S: AsRef<[u8]>> {
    branch: &'branch ExtentBranch<S>,
    pos: usize,
}

impl<'branch, S: AsRef<[u8]>> Iterator for ExtentBranchIter<'branch, S> {
    type Item = ExtentIndex<&'branch [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.branch.subtree_count() {
            let index = ExtentIndex::new(
                &self.branch.subtrees_storage()[self.pos * index::layout::SIZE.unwrap()..]
                    [..index::layout::SIZE.unwrap()],
            );
            self.pos += 1;
            Some(index)
        } else {
            None
        }
    }
}

impl<'branch, S: AsRef<[u8]>> FusedIterator for ExtentBranchIter<'branch, S> {}

#[derive(PartialEq, Clone, Hash)]
pub struct ExtentLeaf<S: AsRef<[u8]>>(ExtentTreeInternal<S>);

impl<S: AsRef<[u8]>> ExtentLeaf<S> {
    pub fn valid(&self) -> bool {
        self.0.valid()
    }

    pub fn extent_count(&self) -> usize {
        self.0.entry_count()
    }

    pub fn max_extents(&self) -> usize {
        self.0.max_entries()
    }

    pub fn iter_extents(&'_ self) -> ExtentLeafIter<'_, S> {
        ExtentLeafIter { leaf: self, pos: 0 }
    }

    fn extents_storage(&self) -> &[u8] {
        self.0.entries_storage()
    }

    pub fn generation(&self) -> u32 {
        self.0.generation()
    }

    pub fn to_owned(&self) -> ExtentLeaf<Vec<u8>> {
        ExtentLeaf(self.0.to_owned())
    }
}

#[derive(PartialEq, Clone, Hash)]
pub struct ExtentLeafIter<'leaf, S: AsRef<[u8]>> {
    leaf: &'leaf ExtentLeaf<S>,
    pos: usize,
}

impl<'leaf, S: AsRef<[u8]>> Iterator for ExtentLeafIter<'leaf, S> {
    type Item = Extent<&'leaf [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.leaf.extent_count() {
            let extent = Extent::new(
                &self.leaf.extents_storage()[self.pos * extent::layout::SIZE.unwrap()..]
                    [..extent::layout::SIZE.unwrap()],
            );
            self.pos += 1;
            Some(extent)
        } else {
            None
        }
    }
}

impl<'leaf, S: AsRef<[u8]>> FusedIterator for ExtentLeafIter<'leaf, S> {}

binary_layout!(layout, LittleEndian, {
    header: NestedExtentHeader,
    entries_and_tail: [u8]
});

#[derive(PartialEq, Clone, Hash)]
struct ExtentTreeInternal<S: AsRef<[u8]>> {
    storage: S,
    checksum_seed: Option<u32>,
}

impl<'inode> ExtentTreeInternal<&'inode [u8]> {
    pub fn from_inode(inode: &'inode Inode) -> Self {
        Self {
            storage: inode.blocks_buffer(),
            checksum_seed: None,
        }
    }
}

impl<S: AsRef<[u8]>> ExtentTreeInternal<S> {
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

        let entries_size = self.entries_size();

        let tail_size = match self.checksum_seed {
            Some(_) => tail::layout::SIZE.unwrap(),
            None => 0,
        };

        if self.view().entries_and_tail().as_ref().len() < entries_size + tail_size {
            return false;
        }

        match self.checksum_seed {
            None => true,
            Some(checksum_seed) => {
                let expected_checksum = {
                    let tail_offset = layout::entries_and_tail::OFFSET + entries_size;

                    let crc = Crc::<u32>::new(&EXT4_CRC32C);
                    let mut digest = crc.digest_with_initial(checksum_seed);
                    digest.update(&self.storage.as_ref()[..tail_offset]);
                    digest.finalize()
                };

                tail::layout::View::new(&self.view().entries_and_tail()[entries_size..])
                    .checksum()
                    .read()
                    == expected_checksum
            }
        }
    }

    pub fn entry_count(&self) -> usize {
        usize::from(self.view().header().entries().read())
    }

    pub fn max_entries(&self) -> usize {
        usize::from(self.view().header().max().read())
    }

    pub fn depth(&self) -> u16 {
        self.view().header().depth().read()
    }

    pub fn generation(&self) -> u32 {
        self.view().header().generation().read()
    }

    pub fn entries_storage(&self) -> &[u8] {
        &self.view().into_entries_and_tail().into_slice()[..self.entries_size()]
    }

    pub fn to_owned(&self) -> ExtentTreeInternal<Vec<u8>> {
        ExtentTreeInternal {
            storage: self.storage.as_ref().to_owned(),
            checksum_seed: self.checksum_seed,
        }
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.storage.as_ref())
    }

    fn entries_size(&self) -> usize {
        self.entry_count() * index::layout::SIZE.unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::ext4::extent::tree::ExtentTree;
    use crate::ext4::extent::{extent, index};
    use crate::ext4::inode::Inode;
    use crate::ext4::superblock::Superblock;
    use crate::ext4::units::{BlockCount, FileBlockNumber, FsBlockNumber, InodeNumber};
    use itertools::Itertools;

    const SUPERBLOCK: &[u8] = include_bytes!("test_data/superblock");
    const ROOT_INODE: &[u8] = include_bytes!("test_data/root_inode");

    #[test]
    pub fn extent_and_index_are_the_same_size() {
        assert_eq!(extent::layout::SIZE.unwrap(), index::layout::SIZE.unwrap());
    }

    #[test]
    fn root_fields() {
        let superblock = Superblock::new(SUPERBLOCK);
        let inode = Inode::new(&superblock, InodeNumber(2), ROOT_INODE);
        let tree = ExtentTree::from_inode(&inode);
        assert!(tree.valid());
        match tree {
            ExtentTree::Branch(_) => panic!("Expected ExtentTree::Leaf"),
            ExtentTree::Leaf(leaf) => {
                assert_eq!(leaf.max_extents(), 4);
                assert_eq!(leaf.generation(), 0);
            }
        }
    }

    #[test]
    pub fn root_iter() {
        let superblock = Superblock::new(SUPERBLOCK);
        let inode = Inode::new(&superblock, InodeNumber(2), ROOT_INODE);
        match ExtentTree::from_inode(&inode) {
            ExtentTree::Branch(_) => panic!("Expected ExtentTree::Leaf"),
            ExtentTree::Leaf(leaf) => {
                let extents = leaf.iter_extents().collect_vec();
                assert_eq!(extents.len(), 1);
                assert!(extents[0].initialized());
                assert_eq!(extents[0].first_file_block_number(), FileBlockNumber(0));
                assert_eq!(extents[0].length(), BlockCount(1));
                assert_eq!(extents[0].first_fs_block_number(), FsBlockNumber(15));
            }
        }
    }
}
