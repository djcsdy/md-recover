use crate::block_device::BlockDevice;
use crate::ext4::directory::Ext4Directory;
use crate::ext4::extent::{Extent, ExtentTree};
use crate::ext4::fs::Ext4Fs;
use crate::ext4::inode::{FileType, Inode};
use crate::ext4::regular_file::Ext4RegularFile;
use crate::ext4::units::{BlockCount, FileBlockNumber};
use std::io;

pub enum Ext4File<D: BlockDevice> {
    Directory(Ext4Directory<D>),
    RegularFile(Ext4RegularFile<D>),
    Unsupported(FileType),
}

impl<D: BlockDevice> Ext4File<D> {
    pub(super) fn from_inode(fs: Ext4Fs<D>, inode: Inode) -> Option<Self> {
        let file_type = inode.file_type();
        Some(match file_type {
            FileType::RegularFile => Self::RegularFile(Ext4RegularFile::from_inode(fs, inode)?),
            FileType::Directory => Self::Directory(Ext4Directory::from_inode(fs, inode)?),
            file_type => Self::Unsupported(file_type),
        })
    }
}

pub(in crate::ext4) struct Ext4FileInternal<D: BlockDevice> {
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

impl<D: BlockDevice> Ext4FileInternal<D> {
    pub(super) fn from_inode(fs: Ext4Fs<D>, inode: Inode) -> Option<Self> {
        let tree = ExtentTree::from_inode(&inode)?.to_owned();

        Some(Self {
            fs,
            inode,
            read_stack: vec![ReadStackEntry::Tree { tree, pos: 0 }],
            block_pos: FileBlockNumber(0),
            byte_pos: 0,
        })
    }

    pub fn block_size(&mut self) -> usize {
        self.fs.block_size()
    }

    pub fn read_next_block(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() < self.fs.block_size() {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }

        loop {
            match self.read_stack.pop() {
                None => {
                    return if self.byte_pos == self.inode.file_size_bytes() {
                        Ok(0)
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
                                let mut subtree_block = vec![0; self.fs.block_size()];
                                self.fs.read_block(index.leaf(), &mut subtree_block)?;
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

                        self.fs
                            .read_block(extent.first_fs_block_number() + pos, buf)?;
                        let bytes_read =
                            usize::try_from(self.inode.file_size_bytes() - self.byte_pos)
                                .unwrap_or(usize::MAX)
                                .clamp(0, self.fs.block_size());

                        self.read_stack.push(ReadStackEntry::Extent {
                            extent,
                            pos: pos + BlockCount(1),
                        });

                        self.byte_pos += u64::try_from(bytes_read).unwrap();
                        self.block_pos += BlockCount(1);

                        return Ok(bytes_read);
                    }
                }
            }
        }
    }
}
