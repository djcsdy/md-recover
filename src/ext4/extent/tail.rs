use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    checksum: u32,
});

pub struct ExtentTail<S: AsRef<[u8]>>(layout::View<S>);

impl<S: AsRef<[u8]>> ExtentTail<S> {
    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }

    pub fn checksum(&self) -> u32 {
        self.0.checksum().read()
    }
}
