use crate::lvm::metadata::MetadataValue;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct MetadataNode {
    pub object: Option<HashMap<String, MetadataNode>>,
    pub array: Option<Vec<MetadataValue>>,
}

impl MetadataNode {
    pub fn empty() -> Self {
        Self {
            object: None,
            array: None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        self.array.as_ref()?.first()?.as_str()
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.array.as_ref()?.first()?.as_u64()
    }

    pub fn as_array(&self) -> Option<&[MetadataValue]> {
        Some(self.array.as_ref()?.as_slice())
    }

    pub fn as_object(&self) -> Option<&HashMap<String, MetadataNode>> {
        self.object.as_ref()
    }
}

impl From<&str> for MetadataNode {
    fn from(value: &str) -> Self {
        Self {
            object: None,
            array: Some(vec![value.into()]),
        }
    }
}

impl From<u64> for MetadataNode {
    fn from(value: u64) -> Self {
        Self {
            object: None,
            array: Some(vec![value.into()]),
        }
    }
}

impl From<Vec<MetadataValue>> for MetadataNode {
    fn from(value: Vec<MetadataValue>) -> Self {
        Self {
            object: None,
            array: Some(value.into()),
        }
    }
}

impl From<HashMap<String, MetadataNode>> for MetadataNode {
    fn from(value: HashMap<String, MetadataNode>) -> Self {
        Self {
            object: Some(value),
            array: None,
        }
    }
}

impl<const N: usize> From<[(String, MetadataNode); N]> for MetadataNode {
    fn from(value: [(String, MetadataNode); N]) -> Self {
        Self {
            object: Some(value.into()),
            array: None,
        }
    }
}
