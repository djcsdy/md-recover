use crate::ext::WideUnsigned;
use crate::ext4::units::{FileBlockNumber, FsBlockNumber};
use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    block: u32,
    leaf_low: u32,
    leaf_high: u16,
    unused: u16,
});

pub struct ExtentIndex<S: AsRef<[u8]>>(layout::View<S>);

impl<S: AsRef<[u8]>> ExtentIndex<S> {
    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }

    pub fn block(&self) -> FileBlockNumber {
        FileBlockNumber(self.0.block().read())
    }

    pub fn leaf(&self) -> FsBlockNumber {
        FsBlockNumber(u64::from_low_high(
            self.0.leaf_low().read(),
            u32::from(self.0.leaf_high().read()),
        ))
    }
}
