use crate::block_device::BlockDevice;
use crate::md::definition::MdArrayDefinition;
use crate::md::diagnosis::Diagnosis;
use crate::md::MdDevice;
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
        Self {
            definition: Rc::new(MdArrayDefinition::new(devices)),
        }
    }

    pub fn diagnose(&self) -> Diagnosis {
        self.definition.diagnose()
    }
}
