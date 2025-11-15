use crate::ext4::inode::{FileType, Permissions};
use binary_layout::LayoutAs;
use derive_more::with_trait::{From, Into};
use std::convert::Infallible;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, From, Into)]
pub struct FileMode(u16);

impl FileMode {
    pub fn from_file_type_and_permissions(file_type: FileType, permissions: Permissions) -> Self {
        Self(
            (u16::from(u8::from(file_type)) << 12)
                | permissions.intersection(Permissions::all()).bits(),
        )
    }

    pub fn file_type(&self) -> FileType {
        FileType::from((self.0 >> 12) as u8)
    }

    pub fn permissions(&self) -> Permissions {
        Permissions::from_bits_truncate(self.0)
    }
}

impl LayoutAs<u16> for FileMode {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u16) -> Result<Self, Self::ReadError> {
        Ok(v.into())
    }

    fn try_write(v: Self) -> Result<u16, Self::WriteError> {
        Ok(v.into())
    }
}

#[cfg(test)]
mod test {
    use crate::ext4::inode::{FileMode, FileType, Permissions};

    #[test]
    fn from_file_type_and_permissions() {
        assert_eq!(
            FileMode::from_file_type_and_permissions(
                FileType::Directory,
                Permissions::USER_READ | Permissions::USER_WRITE | Permissions::USER_EXECUTE
            ),
            FileMode::from(0x41c0)
        );

        assert_eq!(
            FileMode::from_file_type_and_permissions(
                FileType::Socket,
                Permissions::from_bits_retain(0xffff)
            ),
            FileMode::from(0xcfff)
        );
    }
}
