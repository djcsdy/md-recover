use crate::lvm::metadata::location::MetadataLocation;
use binary_layout::binary_layout;
use std::io;
use std::io::Read;

binary_layout!(layout, LittleEndian, {
    checksum: u32,
    magic: [u8; 16],
    version: u32,
    start: u64,
    size: u64,
});

#[derive(Clone)]
pub struct MetadataAreaHeader<S: AsRef<[u8]>> {
    storage: S,
    metadata_locations: Vec<MetadataLocation<Vec<u8>>>,
}

impl<S: AsRef<[u8]>> MetadataAreaHeader<S> {
    pub fn checksum(&self) -> u32 {
        self.view().checksum().read()
    }

    pub fn magic(&self) -> [u8; 16] {
        *self.view().magic()
    }

    pub fn version(&self) -> u32 {
        self.view().version().read()
    }

    pub fn start(&self) -> u64 {
        self.view().start().read()
    }

    pub fn size(&self) -> u64 {
        self.view().size().read()
    }

    pub fn metadata_locations(&self) -> &Vec<MetadataLocation<Vec<u8>>> {
        &self.metadata_locations
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.storage.as_ref())
    }
}

impl MetadataAreaHeader<Vec<u8>> {
    pub fn read(mut reader: impl Read) -> io::Result<Self> {
        let mut storage = vec![0u8; layout::SIZE.unwrap()];
        reader.read_exact(&mut storage)?;

        let metadata_locations = MetadataLocation::read_all(&mut reader)?;

        Ok(Self {
            storage,
            metadata_locations,
        })
    }
}
