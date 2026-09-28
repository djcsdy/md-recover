#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct MetadataSyntaxError {
    pub bytes: Vec<u8>,
}
