use crate::block_device::BlockDevice;
use crate::md::superblock::Superblock;
use crate::md::MdDevice;

pub struct MdArray {
    devices: Vec<MdDevice<Box<dyn Superblock>, Box<dyn BlockDevice>>>,
}

impl MdArray {
    pub fn new(devices: Vec<MdDevice<Box<dyn Superblock>, Box<dyn BlockDevice>>>) -> Self {
        Self { devices }
    }
}
