use binary_layout::binary_layout;
use std::io;
use std::io::Read;

binary_layout!(layout, LittleEndian, {
    offset: u64,
    size: u64,
    checksum: u32,
    flags: u32,
});

#[derive(Clone)]
pub struct MetadataLocation<S: AsRef<[u8]>>(S);

impl<S: AsRef<[u8]>> MetadataLocation<S> {
    pub fn new(storage: S) -> Self {
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

impl MetadataLocation<Vec<u8>> {
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut buf = vec![0u8; layout::SIZE.unwrap()];
        reader.read_exact(&mut buf)?;
        Ok(Self::new(buf))
    }

    pub fn read_all<R: Read>(mut reader: R) -> io::Result<Vec<Self>> {
        let mut locations = Vec::with_capacity(1);
        loop {
            let location = Self::read(&mut reader)?;
            if location.is_end_marker() {
                return Ok(locations);
            } else {
                locations.push(location);
            }
        }
    }
}
