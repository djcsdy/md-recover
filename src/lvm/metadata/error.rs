use thiserror::Error;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Error)]
pub enum MetadataError {
    #[error("expected exactly one volume group object")]
    InvalidVolumeGroup,
    #[error("invalid volume group UUID")]
    InvalidUuid,
    #[error("invalid sequence number")]
    InvalidSequenceNumber,
    #[error("invalid LVM metadata syntax")]
    Syntax,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Error)]
#[error("invalid LVM metadata syntax")]
pub struct MetadataSyntaxError {
    pub bytes: Vec<u8>,
}
