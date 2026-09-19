#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct MetadataParseError {
    pub bytes: Vec<u8>,
}
