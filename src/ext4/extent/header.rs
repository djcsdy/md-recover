use binary_layout::binary_layout;

binary_layout!(layout, LittleEndian, {
    magic: u16,
    entries: u16,
    max: u16,
    depth: u16,
    generation: u32,
});

pub struct ExtentHeader<S: AsRef<[u8]>>(layout::View<S>);

impl<S: AsRef<[u8]>> ExtentHeader<S> {
    const MAGIC: u16 = 0xf30a;

    pub fn new(storage: S) -> Self {
        Self(layout::View::new(storage))
    }

    pub fn valid(&self) -> bool {
        self.magic() == Self::MAGIC
    }

    fn magic(&self) -> u16 {
        self.0.magic().read()
    }

    pub fn entries(&self) -> u16 {
        self.0.entries().read()
    }

    pub fn max(&self) -> u16 {
        self.0.max().read()
    }

    pub fn depth(&self) -> u16 {
        self.0.depth().read()
    }

    pub fn generation(&self) -> u32 {
        self.0.generation().read()
    }
}

#[cfg(test)]
mod test {
    use crate::ext4::extent::header::ExtentHeader;

    const HEADER: &[u8] = include_bytes!("test_data/header");

    #[test]
    fn parse() {
        let header = ExtentHeader::new(HEADER);
        assert!(header.valid());
        assert_eq!(header.entries(), 1);
        assert_eq!(header.max(), 4);
        assert_eq!(header.depth(), 0);
        assert_eq!(header.generation(), 0);
    }
}
