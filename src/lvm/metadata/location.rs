use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    offset: u64,
    size: u64,
    checksum: u32,
    flags: u32,
});

#[derive(Clone)]
pub struct MetadataLocation([u8; Self::SIZE_BYTES]);

impl MetadataLocation {
    pub const SIZE_BYTES: usize = layout::SIZE.unwrap();

    pub fn new(storage: [u8; Self::SIZE_BYTES]) -> Self {
        Self(storage)
    }

    pub fn offset(&self) -> u64 {
        self.view().offset().read()
    }

    pub fn size(&self) -> u64 {
        self.view().size().read()
    }

    pub fn checksum(&self) -> u32 {
        self.view().checksum().read()
    }

    pub fn flags(&self) -> u32 {
        self.view().flags().read()
    }

    pub fn is_end_marker(&self) -> bool {
        self.offset() == 0 && self.size() == 0
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.0.as_ref())
    }
}
