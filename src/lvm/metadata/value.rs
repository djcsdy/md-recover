#[derive(Eq, PartialEq, Clone, Debug)]
pub enum MetadataValue {
    String(String),
    Integer(u64),
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
}

impl From<&str> for MetadataValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<u64> for MetadataValue {
    fn from(value: u64) -> Self {
        Self::Integer(value)
    }
}
