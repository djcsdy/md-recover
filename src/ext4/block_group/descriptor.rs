use crate::ext::WideUnsigned;
use crate::ext4::block_group::Flags;
use crate::ext4::units::FsBlockIndex;
use binary_layout::binary_layout;

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
});

pub struct BlockGroupDescriptor([u8; layout::SIZE.unwrap()]);

impl BlockGroupDescriptor {
    pub fn new<B: AsRef<[u8]>>(buffer: B) -> Self {
        let mut storage = [0u8; layout::SIZE.unwrap()];
        let source_length = buffer.as_ref().len().clamp(0, layout::SIZE.unwrap());
        storage[..source_length].copy_from_slice(&buffer.as_ref()[..source_length]);
        Self(storage)
    }

    pub fn block_bitmap_block(&self) -> FsBlockIndex {
        FsBlockIndex(u64::from_low_high(
            self.view().block_bitmap_block_low().read(),
            self.view().block_bitmap_block_high().read(),
        ))
    }

    pub fn inode_bitmap_block(&self) -> FsBlockIndex {
        FsBlockIndex(u64::from_low_high(
            self.view().inode_bitmap_block_low().read(),
            self.view().inode_bitmap_block_high().read(),
        ))
    }

    pub fn inode_table_block(&self) -> FsBlockIndex {
        FsBlockIndex(u64::from_low_high(
            self.view().inode_table_block_low().read(),
            self.view().inode_table_block_high().read(),
        ))
    }

    pub fn free_block_count(&self) -> u32 {
        u32::from_low_high(
            self.view().free_block_count_low().read(),
            self.view().free_block_count_high().read(),
        )
    }

    pub fn free_inode_count(&self) -> u32 {
        u32::from_low_high(
            self.view().free_inode_count_low().read(),
            self.view().free_inode_count_high().read(),
        )
    }
    pub fn used_directories_count(&self) -> u32 {
        u32::from_low_high(
            self.view().used_directories_count_low().read(),
            self.view().used_directories_count_high().read(),
        )
    }

    pub fn flags(&self) -> Flags {
        self.view().flags().read()
    }

    pub fn exclude_bitmap_block(&self) -> FsBlockIndex {
        FsBlockIndex(u64::from_low_high(
            self.view().exclude_bitmap_block_low().read(),
            self.view().exclude_bitmap_block_high().read(),
        ))
    }

    pub fn block_bitmap_checksum(&self) -> u32 {
        u32::from_low_high(
            self.view().block_bitmap_checksum_low().read(),
            self.view().block_bitmap_checksum_high().read(),
        )
    }

    pub fn inode_bitmap_checksum(&self) -> u32 {
        u32::from_low_high(
            self.view().inode_bitmap_checksum_low().read(),
            self.view().inode_bitmap_checksum_high().read(),
        )
    }

    pub fn unused_inode_count(&self) -> u32 {
        u32::from_low_high(
            self.view().unused_inode_count_low().read(),
            self.view().unused_inode_count_high().read(),
        )
    }

    pub fn checksum(&self) -> u16 {
        self.view().checksum().read()
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.0.as_ref())
    }
}
