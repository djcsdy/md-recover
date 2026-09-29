use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum MetadataValue {
    String(String),
    Integer(u64),
    Array(Vec<MetadataValue>),
    Object(HashMap<String, MetadataValue>),
}

impl MetadataValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            MetadataValue::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            MetadataValue::Integer(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[MetadataValue]> {
        match self {
            MetadataValue::Array(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, MetadataValue>> {
        match self {
            MetadataValue::Object(value) => Some(value),
            _ => None,
        }
    }
}
