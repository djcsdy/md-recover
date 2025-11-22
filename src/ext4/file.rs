use crate::block_device::BlockDevice;
use crate::ext4::extent::{Extent, ExtentTree};
use crate::ext4::fs::Ext4Fs;
use crate::ext4::inode::Inode;
use crate::ext4::units::{BlockCount, FileBlockNumber};
use std::io;

pub struct Ext4File<D: BlockDevice> {
    fs: Ext4Fs<D>,
    inode: Inode,
    read_stack: Vec<ReadStackEntry>,
    block_pos: FileBlockNumber,
    byte_pos: u64,
}

enum ReadStackEntry {
    Tree {
        tree: ExtentTree<Vec<u8>>,
        pos: usize,
    },
    Extent {
        extent: Extent<Vec<u8>>,
        pos: BlockCount<u16>,
    },
}

impl<D: BlockDevice> Ext4File<D> {
    pub(super) fn new(fs: Ext4Fs<D>, inode: Inode) -> Option<Self> {
        let tree = ExtentTree::from_inode(&inode)?.to_owned();

        Some(Self {
            fs,
            inode,
            read_stack: vec![ReadStackEntry::Tree { tree, pos: 0 }],
            block_pos: FileBlockNumber(0),
            byte_pos: 0,
        })
    }

    pub fn read_next_block(&mut self) -> io::Result<Option<Vec<u8>>> {
        loop {
            match self.read_stack.pop() {
                None => {
                    return if self.byte_pos == self.inode.file_size_bytes() {
                        Ok(None)
                    } else {
                        Err(io::Error::from(io::ErrorKind::UnexpectedEof))
                    }
                }
                Some(ReadStackEntry::Tree { tree, pos }) => {
                    if pos < tree.entry_count() {
                        match tree {
                            ExtentTree::Branch(branch) => {
                                let index = branch.subtree_index_at(pos).unwrap();
                                if index.block() != self.block_pos {
                                    return Err(io::Error::from(io::ErrorKind::InvalidData));
                                }
                                let subtree_block = self.fs.read_block(index.leaf())?;
                                let subtree = ExtentTree::from_block(&self.inode, subtree_block);
                                self.read_stack.push(ReadStackEntry::Tree {
                                    tree: ExtentTree::Branch(branch),
                                    pos: pos + 1,
                                });
                                self.read_stack.push(ReadStackEntry::Tree {
                                    tree: subtree,
                                    pos: 0,
                                });
                            }
                            ExtentTree::Leaf(leaf) => {
                                let extent = leaf.extent_at(pos).unwrap().into_owned();
                                if extent.first_file_block_number() != self.block_pos {
                                    return Err(io::Error::from(io::ErrorKind::InvalidData));
                                }
                                self.read_stack.push(ReadStackEntry::Tree {
                                    tree: ExtentTree::Leaf(leaf),
                                    pos: pos + 1,
                                });
                                self.read_stack.push(ReadStackEntry::Extent {
                                    extent,
                                    pos: BlockCount(0),
                                });
                            }
                        }
                    }
                }
                Some(ReadStackEntry::Extent { extent, pos }) => {
                    if pos < extent.length() {
                        if self.byte_pos >= self.inode.file_size_bytes() {
                            return Err(io::Error::from(io::ErrorKind::InvalidData));
                        }

                        let mut block = self.fs.read_block(extent.first_fs_block_number() + pos)?;
                        block.truncate(
                            block.len().clamp(
                                0,
                                usize::try_from(self.inode.file_size_bytes() - self.byte_pos)
                                    .unwrap_or(usize::MAX),
                            ),
                        );

                        self.read_stack.push(ReadStackEntry::Extent {
                            extent,
                            pos: pos + BlockCount(1),
                        });

                        self.byte_pos += u64::try_from(block.len()).unwrap();
                        self.block_pos += BlockCount(1);

                        return Ok(Some(block));
                    }
                }
            }
        }
    }
}
