use crate::ext::WideUnsigned;
use crate::ext4::extent::ExtentLookup;
use crate::ext4::units::{BlockCount, FileBlockIndex, FsBlockIndex};
use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    first_file_block_index: FileBlockIndex as u32,
    length: u16,
    first_fs_block_index_high: u16,
    first_fs_block_index_low: u32,
});

pub struct Extent<S: AsRef<[u8]>>(layout::View<S>);

impl<S: AsRef<[u8]>> Extent<S> {
    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }

    pub fn first_file_block_index(&self) -> FileBlockIndex {
        self.0.first_file_block_index().read()
    }

    pub fn initialized(&self) -> bool {
        self.0.length().read() & 0x8000 == 0x8000
    }

    pub fn length(&self) -> BlockCount<u16> {
        BlockCount(self.0.length().read() & 0x7fff)
    }

    pub fn first_fs_block_index(&self) -> FsBlockIndex {
        FsBlockIndex(u64::from_low_high(
            self.0.first_fs_block_index_low().read(),
            u32::from(self.0.first_fs_block_index_high().read()),
        ))
    }

    pub fn lookup(&self, file_block_index: FileBlockIndex) -> ExtentLookup {
        if file_block_index < self.first_file_block_index() {
            ExtentLookup::OutOfBounds
        } else {
            let index_in_range = *(file_block_index - self.first_file_block_index());
            if index_in_range >= u32::from(*self.length()) {
                ExtentLookup::OutOfBounds
            } else {
                (*self.first_fs_block_index())
                    .checked_add(u64::from(index_in_range))
                    .map(FsBlockIndex)
                    .map(ExtentLookup::Direct)
                    .unwrap_or(ExtentLookup::OutOfBounds)
            }
        }
    }
}
