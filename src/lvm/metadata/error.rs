use thiserror::Error;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Error)]
#[error("invalid LVM metadata syntax")]
pub struct MetadataSyntaxError {
    pub bytes: Vec<u8>,
}
