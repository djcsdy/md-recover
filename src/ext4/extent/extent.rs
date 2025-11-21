use crate::ext::WideUnsigned;
use crate::ext4::units::{BlockCount, FileBlockNumber, FsBlockNumber};
use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    first_file_block_number: FileBlockNumber as u32,
    length: u16,
    first_fs_block_number_high: u16,
    first_fs_block_number_low: u32,
});

pub struct Extent<S: AsRef<[u8]>>(layout::View<S>);

impl<S: AsRef<[u8]>> Extent<S> {
    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }

    pub fn first_file_block_number(&self) -> FileBlockNumber {
        self.0.first_file_block_number().read()
    }

    pub fn initialized(&self) -> bool {
        self.0.length().read() & 0x8000 != 0x8000
    }

    pub fn length(&self) -> BlockCount<u16> {
        BlockCount(self.0.length().read() & 0x7fff)
    }

    pub fn first_fs_block_number(&self) -> FsBlockNumber {
        FsBlockNumber(u64::from_low_high(
            self.0.first_fs_block_number_low().read(),
            u32::from(self.0.first_fs_block_number_high().read()),
        ))
    }
}
