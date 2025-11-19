use crate::ext::WideUnsigned;
use crate::ext4::units::{BlockCount, FileBlockIndex, FsBlockIndex};
use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    block: FileBlockIndex as u32,
    length: u16,
    start_high: u16,
    start_low: u32,
});

pub struct Extent<S: AsRef<[u8]>>(layout::View<S>);

impl<S: AsRef<[u8]>> Extent<S> {
    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }

    pub fn block(&self) -> FileBlockIndex {
        self.0.block().read()
    }

    pub fn initialized(&self) -> bool {
        self.0.length().read() & 0x8000 == 0x8000
    }

    pub fn length(&self) -> BlockCount<u16> {
        BlockCount(self.0.length().read() & 0x7fff)
    }

    pub fn start(&self) -> FsBlockIndex {
        FsBlockIndex(u64::from_low_high(
            self.0.start_low().read(),
            u32::from(self.0.start_high().read()),
        ))
    }
}
