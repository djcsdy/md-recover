use std::ffi::OsStr;
use super::ArrayUuid;

pub trait Superblock {
    fn valid(&self) -> bool;
    fn major_version(&self) -> u32;
    fn array_uuid(&self) -> ArrayUuid;
    fn array_name(&self) -> Option<&OsStr>;
}

impl Superblock for Box<dyn Superblock + '_> {
    fn valid(&self) -> bool {
        (**self).valid()
    }

    fn major_version(&self) -> u32 {
        (**self).major_version()
    }

    fn array_uuid(&self) -> ArrayUuid {
        (**self).array_uuid()
    }

    fn array_name(&self) -> Option<&OsStr> {
        (**self).array_name()
    }
}
