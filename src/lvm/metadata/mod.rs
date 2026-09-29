mod area;
mod area_header;
mod error;
mod location;
mod parse;
mod value;

#[allow(unused_imports)]
pub use self::{
    area::MetadataArea, error::MetadataError, error::MetadataSyntaxError, value::MetadataValue,
};
use itertools::Itertools;
use std::collections::HashMap;
use winnow::error::EmptyError;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Metadata {
    root: HashMap<String, MetadataValue>,
}

impl Metadata {
    pub fn parse<Buffer: AsRef<[u8]>>(buffer: Buffer) -> Result<Self, MetadataSyntaxError<Buffer>> {
        parse::metadata::<_, EmptyError>(&mut buffer.as_ref())
            .map_err(|_| MetadataSyntaxError { buffer })
    }

    pub fn volume_group_name(&self) -> Result<&String, MetadataError> {
        self.volume_group().map(|(name, _)| name)
    }

    pub fn uuid(&self) -> Result<&str, MetadataError> {
        self.volume_group_metadata()
            .and_then(|metadata| metadata.get("uuid").ok_or(MetadataError::InvalidUuid))
            .and_then(|uuid| uuid.as_str().ok_or(MetadataError::InvalidUuid))
    }

    pub fn sequence_number(&self) -> Result<u64, MetadataError> {
        self.volume_group_metadata()
            .and_then(|metadata| {
                metadata
                    .get("seqno")
                    .ok_or(MetadataError::InvalidSequenceNumber)
            })
            .and_then(|sequence_number| {
                sequence_number
                    .as_u64()
                    .ok_or(MetadataError::InvalidSequenceNumber)
            })
    }

    fn volume_group(&self) -> Result<(&String, &HashMap<String, MetadataValue>), MetadataError> {
        self.root
            .iter()
            .exactly_one()
            .map_err(|_| MetadataError::InvalidVolumeGroup)
            .and_then(|(volume_group_name, value)| {
                value
                    .as_object()
                    .map(|metadata| (volume_group_name, metadata))
                    .ok_or(MetadataError::InvalidVolumeGroup)
            })
    }

    fn volume_group_metadata(&self) -> Result<&HashMap<String, MetadataValue>, MetadataError> {
        self.volume_group().map(|(_, metadata)| metadata)
    }
}
