use std::ffi::OsStr;

use crate::md::algorithm::MdAlgorithm;

use super::ArrayUuid;

pub trait Superblock {
    fn valid(&self) -> bool;
    fn major_version(&self) -> u32;
    fn array_uuid(&self) -> ArrayUuid;
    fn array_name(&self) -> Option<&OsStr>;
    fn algorithm(&self) -> MdAlgorithm;
    fn chunk_size(&self) -> u32;
}

impl Superblock for Box<dyn Superblock> {
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

    fn algorithm(&self) -> MdAlgorithm {
        (**self).algorithm()
    }

    fn chunk_size(&self) -> u32 {
        (**self).chunk_size()
    }
}
