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
}
