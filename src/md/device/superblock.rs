use crate::md::superblock::Superblock;

pub enum MdDeviceSuperblock<S: Superblock> {
    Superblock(S),
    TooSmall,
    Missing,
}

impl<S: Superblock> MdDeviceSuperblock<S> {
    pub(crate) fn as_option(&self) -> Option<&S> {
        match self {
            MdDeviceSuperblock::Superblock(superblock) => Some(superblock),
            _ => None,
        }
    }
}
