use crate::block_device::BlockDevice;
use crate::md::MdDevice;
use std::io::{Read, Seek};
use std::rc::Rc;

pub struct MdArrayDefinition<D>
where
    D: BlockDevice + Read + Seek,
{
    pub devices: Vec<Rc<MdDevice<D>>>,
}

impl<D> MdArrayDefinition<D>
where
    D: BlockDevice + Read + Seek,
{
    pub fn new(devices: impl IntoIterator<Item = impl Into<Rc<MdDevice<D>>>>) -> Self {
        Self {
            devices: devices.into_iter().map(Into::into).collect(),
        }
    }
}
