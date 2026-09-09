use crate::lvm::crc::LVM_CRC;
use crate::lvm::metadata::location::MetadataLocation;
use binary_layout::{binary_layout, Field};
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
pub struct MetadataAreaHeader {
    start: u64,
    size: u64,
    metadata_locations: Vec<MetadataLocation>,
}

impl MetadataAreaHeader {
    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn metadata_locations(&self) -> &Vec<MetadataLocation> {
        &self.metadata_locations
    }
}

impl MetadataAreaHeader {
    pub fn read(mut reader: impl Read) -> io::Result<Self> {
        let mut header = vec![0u8; layout::SIZE.unwrap()];
        reader.read_exact(&mut header)?;

        let mut metadata_locations = Vec::with_capacity(1);

        loop {
            let mut buf = [0u8; MetadataLocation::SIZE_BYTES];
            reader.read_exact(&mut buf)?;
            header.extend_from_slice(&buf);

            let metadata_location = MetadataLocation::new(buf);

            if metadata_location.is_end_marker() {
                break;
            } else {
                metadata_locations.push(metadata_location);
            }
        }

        let actual_checksum = LVM_CRC.checksum(&header[layout::magic::OFFSET..]);

        let view = layout::View::new(header);

        if view.magic() != b" LVM2 x[5A%r0N*>"
            || actual_checksum != view.checksum().read()
            || view.version().read() != 1
        {
            return Err(io::ErrorKind::InvalidData.into());
        }

        Ok(Self {
            start: view.start().read(),
            size: view.size().read(),
            metadata_locations,
        })
    }
}
