mod area;
mod area_header;
mod error;
mod location;
mod parse;
mod value;

#[allow(unused_imports)]
pub use self::{area::MetadataArea, value::MetadataValue};
use crate::lvm::metadata::error::MetadataError;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Metadata {
    root: HashMap<String, MetadataValue>,
}

impl Metadata {
    pub fn volume_group_name(&self) -> Result<&String, MetadataError> {
        self.volume_group().map(|(name, _)| name)
    }

    pub fn uuid(&self) -> Result<&String, MetadataError> {
        self.volume_group_metadata()
            .and_then(|metadata| metadata.get("uuid").ok_or(MetadataError::InvalidUuid))
            .and_then(|uuid| match uuid {
                MetadataValue::String(uuid) => Ok(uuid),
                _ => Err(MetadataError::InvalidUuid),
            })
    }

    fn volume_group(&self) -> Result<(&String, &HashMap<String, MetadataValue>), MetadataError> {
        self.root
            .iter()
            .exactly_one()
            .map_err(|_| MetadataError::InvalidVolumeGroup)
            .and_then(|(volume_group_name, value)| match value {
                MetadataValue::Object(metadata) => Ok((volume_group_name, metadata)),
                _ => Err(MetadataError::InvalidVolumeGroup),
            })
    }

    fn volume_group_metadata(&self) -> Result<&HashMap<String, MetadataValue>, MetadataError> {
        self.volume_group().map(|(_, metadata)| metadata)
    }
}
