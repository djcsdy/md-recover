use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum MetadataValue {
    String(String),
    Integer(u64),
    Array(Vec<MetadataValue>),
    Object(HashMap<String, MetadataValue>),
}
