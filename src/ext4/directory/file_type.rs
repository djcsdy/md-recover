use crate::ext4::inode::FileType;
use binary_layout::LayoutAs;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::Infallible;

#[repr(u8)]
#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, IntoPrimitive, FromPrimitive,
)]
pub enum Ext4DirectoryInlineFileType {
    RegularFile = 1,
    Directory = 2,
    CharacterDevice = 3,
    BlockDevice = 4,
    Fifo = 5,
    Socket = 6,
    SymbolicLink = 7,
    #[num_enum(catch_all)]
    Unknown(u8) = 0,
}

impl LayoutAs<u8> for Ext4DirectoryInlineFileType {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u8) -> Result<Self, Self::ReadError> {
        Ok(v.into())
    }

    fn try_write(v: Self) -> Result<u8, Self::WriteError> {
        Ok(v.into())
    }
}

impl From<FileType> for Ext4DirectoryInlineFileType {
    fn from(value: FileType) -> Self {
        match value {
            FileType::Socket => Self::Socket,
            FileType::SymbolicLink => Self::SymbolicLink,
            FileType::RegularFile => Self::RegularFile,
            FileType::BlockDevice => Self::BlockDevice,
            FileType::Directory => Self::Directory,
            FileType::CharacterDevice => Self::CharacterDevice,
            FileType::Fifo => Self::Fifo,
            FileType::Unknown(v) => Self::Unknown(v),
        }
    }
}

impl From<Ext4DirectoryInlineFileType> for FileType {
    fn from(value: Ext4DirectoryInlineFileType) -> Self {
        match value {
            Ext4DirectoryInlineFileType::RegularFile => Self::RegularFile,
            Ext4DirectoryInlineFileType::Directory => Self::Directory,
            Ext4DirectoryInlineFileType::CharacterDevice => Self::CharacterDevice,
            Ext4DirectoryInlineFileType::BlockDevice => Self::BlockDevice,
            Ext4DirectoryInlineFileType::Fifo => Self::Fifo,
            Ext4DirectoryInlineFileType::Socket => Self::Socket,
            Ext4DirectoryInlineFileType::SymbolicLink => Self::SymbolicLink,
            Ext4DirectoryInlineFileType::Unknown(v) => Self::Unknown(v),
        }
    }
}
