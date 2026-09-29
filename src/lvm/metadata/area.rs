use crate::lvm::metadata::area_header::MetadataAreaHeader;
use crate::lvm::metadata::error::MetadataSyntaxError;
use crate::lvm::metadata::Metadata;
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Clone, Debug)]
pub struct MetadataArea {
    metadata: Vec<Result<Metadata, MetadataSyntaxError<Vec<u8>>>>,
}

impl MetadataArea {
    pub fn read(mut reader: impl Read + Seek) -> io::Result<Self> {
        let actual_start = reader.stream_position()?;

        let header = MetadataAreaHeader::read(&mut reader)?;

        if actual_start != header.start() {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let mut metadata = Vec::new();
        for location in header.metadata_locations() {
            reader.seek(SeekFrom::Start(actual_start + location.offset()))?;

            let mut buf = vec![0; usize::try_from(location.size()).unwrap()];
            reader.read_exact(&mut buf)?;

            metadata.push(Metadata::parse(buf))
        }

        Ok(Self { metadata })
    }

    pub fn metadata(&self) -> &Vec<Result<Metadata, MetadataSyntaxError<Vec<u8>>>> {
        &self.metadata
    }
}
