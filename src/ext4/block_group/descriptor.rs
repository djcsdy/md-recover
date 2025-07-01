use crate::ext4::block_group::Flags;
use binary_layout::binary_layout;
use std::io::{Read, Result};

binary_layout!(layout, LittleEndian, {
    block_bitmap_block_low: u32,
    inode_bitmap_block_low: u32,
    inode_table_block_low: u32,
    free_block_count_low: u16,
    free_inode_count_low: u16,
    used_directories_count_low: u16,
    flags: Flags as u16,
    exclude_bitmap_block_low: u32,
    block_bitmap_checksum_low: u16,
    inode_bitmap_checksum_low: u16,
    unused_inode_count_low: u16,
    checksum: u16,
    block_bitmap_block_high: u32,
    inode_bitmap_block_high: u32,
    inode_table_block_high: u32,
    free_block_count_high: u16,
    free_inode_count_high: u16,
    used_directories_count_high: u16,
    unused_inode_count_high: u16,
    exclude_bitmap_block_high: u32,
    block_bitmap_checksum_high: u16,
    inode_bitmap_checksum_high: u16,
    reserved: u32,
});

pub struct BlockGroupDescriptor<S: AsRef<[u8]>>(S);

impl<S: AsRef<[u8]>> BlockGroupDescriptor<S> {
    pub fn new(storage: S) -> Self {
        Self(storage)
    }

    pub fn block_bitmap_block(&self) -> u64 {
        u64::from(self.view().block_bitmap_block_low().read())
            | (u64::from(self.view().block_bitmap_block_high().read()) << 32)
    }

    pub fn inode_bitmap_block(&self) -> u64 {
        u64::from(self.view().inode_bitmap_block_low().read())
            | (u64::from(self.view().inode_bitmap_block_high().read()) << 32)
    }

    pub fn inode_table_block(&self) -> u64 {
        u64::from(self.view().inode_table_block_low().read())
            | (u64::from(self.view().inode_table_block_high().read()) << 32)
    }

    pub fn free_block_count(&self) -> u32 {
        u32::from(self.view().free_block_count_low().read())
            | (u32::from(self.view().free_block_count_high().read()) << 16)
    }

    pub fn free_inode_count(&self) -> u32 {
        u32::from(self.view().free_inode_count_low().read())
            | (u32::from(self.view().free_inode_count_high().read()) << 16)
    }

    pub fn used_directories_count(&self) -> u32 {
        u32::from(self.view().used_directories_count_low().read())
            | (u32::from(self.view().used_directories_count_high().read()) << 16)
    }

    pub fn flags(&self) -> Flags {
        self.view().flags().read()
    }

    pub fn exclude_bitmap_block(&self) -> u64 {
        u64::from(self.view().exclude_bitmap_block_low().read())
            | (u64::from(self.view().exclude_bitmap_block_high().read()) << 32)
    }

    pub fn block_bitmap_checksum(&self) -> u32 {
        u32::from(self.view().block_bitmap_checksum_low().read())
            | (u32::from(self.view().block_bitmap_checksum_high().read()) << 16)
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.0.as_ref())
    }
}

impl BlockGroupDescriptor<Vec<u8>> {
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        let mut buf = vec![0u8; layout::SIZE.unwrap()];
        reader.read_exact(&mut buf)?;
        Ok(Self::new(buf))
    }
}
