use crate::block_device::BlockDevice;
use crate::ext4::file::Ext4FileInternal;
use crate::ext4::inode::Inode;
use crate::ext4::Ext4Fs;
use std::io;

pub struct Ext4RegularFile<D: BlockDevice>(Ext4FileInternal<D>);

impl<D: BlockDevice> Ext4RegularFile<D> {
    pub(in crate::ext4) fn open(fs: Ext4Fs<D>, inode: Inode) -> io::Result<Self> {
        Ok(Self(Ext4FileInternal::open(fs, inode)?))
    }

    pub fn file_size_bytes(&self) -> u64 {
        self.0.file_size_bytes()
    }

    pub fn read_next_block(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read_next_block(buf)
    }
}
