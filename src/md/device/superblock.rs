use crate::md::superblock::Superblock;

pub enum MdDeviceSuperblock {
    Superblock(Box<dyn Superblock>),
    TooSmall,
    Missing,
}

impl MdDeviceSuperblock {
    pub(crate) fn as_option(&self) -> Option<&dyn Superblock> {
        match self {
            MdDeviceSuperblock::Superblock(superblock) => Some(superblock.as_ref()),
            _ => None,
        }
    }
}
