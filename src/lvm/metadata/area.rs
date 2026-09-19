use crate::lvm::metadata::area_header::MetadataAreaHeader;
use crate::lvm::metadata::error::MetadataParseError;
use crate::lvm::metadata::{parse, MetadataValue};
use std::collections::HashMap;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use winnow::error::EmptyError;

#[derive(Clone, Debug)]
pub struct MetadataArea {
    expected_start: u64,
    actual_start: u64,
    metadata: Vec<Result<HashMap<String, MetadataValue>, MetadataParseError>>,
}

impl MetadataArea {
    pub fn read(mut reader: impl Read + Seek) -> io::Result<Self> {
        let actual_start = reader.stream_position()?;

        let header = MetadataAreaHeader::read(&mut reader)?;

        let mut metadata = Vec::new();
        for location in header.metadata_locations() {
            reader.seek(SeekFrom::Start(actual_start + location.offset()))?;

            let mut buf = vec![0; usize::try_from(location.size()).unwrap()];
            reader.read_exact(&mut buf)?;

            metadata.push(
                parse::metadata::<_, EmptyError>(&mut buf.as_slice())
                    .map_err(|_| MetadataParseError { bytes: buf }),
            )
        }

        Ok(Self {
            expected_start: header.start(),
            actual_start,
            metadata,
        })
    }
}
