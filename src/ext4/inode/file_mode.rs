use crate::ext4::inode::{FileType, Permissions};
use bitflags::Flags;
use derive_more::with_trait::{From, Into};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into)]
pub struct FileMode(u16);

impl FileMode {
    pub fn from_file_type_and_permissions(file_type: FileType, permissions: Permissions) -> Self {
        Self(
            (u16::from(u8::from(file_type)) << 12)
                & permissions.intersection(Permissions::all()).bits(),
        )
    }

    pub fn file_type(&self) -> FileType {
        FileType::from((self.0 >> 12) as u8)
    }

    pub fn permissions(&self) -> Permissions {
        Permissions::from_bits_truncate(self.0)
    }
}
