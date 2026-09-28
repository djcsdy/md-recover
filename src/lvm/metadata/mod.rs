mod area;
mod area_header;
mod error;
mod location;
mod parse;
mod value;

#[allow(unused_imports)]
pub use self::{area::MetadataArea, value::MetadataValue};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Metadata {
    root: HashMap<String, MetadataValue>,
}
