use std::ffi::OsStr;

use super::ArrayUuid;
use crate::md::algorithm::MdAlgorithm;
use crate::md::superblock::reshape_status::ReshapeStatus;

pub trait Superblock {
    fn valid(&self) -> bool;
    fn major_version(&self) -> u32;
    fn array_uuid(&self) -> ArrayUuid;
    fn array_name(&self) -> Option<&OsStr>;
    fn algorithm(&self) -> MdAlgorithm;
    fn size(&self) -> u64;
    fn chunk_size(&self) -> u32;
    fn reshape_status(&self) -> ReshapeStatus;
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

    fn size(&self) -> u64 {
        (**self).size()
    }

    fn chunk_size(&self) -> u32 {
        (**self).chunk_size()
    }

    fn reshape_status(&self) -> ReshapeStatus {
        (**self).reshape_status()
    }
}
