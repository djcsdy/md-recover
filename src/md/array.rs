use crate::block_device::BlockDevice;
use crate::md::config::MdConfig;
use crate::md::definition::MdArrayDefinition;
use crate::md::diagnosis::Diagnosis;
use crate::md::MdDevice;
use itertools::Itertools;
use std::io::{Read, Seek};
use std::rc::Rc;

pub struct MdArray<D>
where
    D: BlockDevice + Read + Seek,
{
    definition: Rc<MdArrayDefinition<D>>,
}

impl<D> MdArray<D>
where
    D: BlockDevice + Read + Seek,
{
    pub fn open(devices: impl IntoIterator<Item = impl Into<Rc<MdDevice<D>>>>) -> Self {
        let devices = devices.into_iter().map(Into::into).collect_vec();
        let config =
            MdConfig::from_superblocks(devices.iter().map(|device| device.superblock.clone()));
        Self {
            definition: Rc::new(MdArrayDefinition { devices, config }),
        }
    }

    pub fn diagnose(&self) -> Diagnosis {
        self.definition.diagnose()
    }
}
