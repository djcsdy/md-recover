use crate::block_device::BlockDevice;

pub struct Ext4Fs<D: BlockDevice> {
    device: D,
}

impl<D: BlockDevice> Ext4Fs<D> {
    pub fn open(device: D) -> Self {
        Self { device }
    }
}
