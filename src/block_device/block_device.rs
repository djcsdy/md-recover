use crate::block_device::native::NativeBlockDevice;
use std::fs::File;
use std::io::Result;
use std::path::Path;

pub trait BlockDevice {
    fn open_path<P: AsRef<Path>>(path: P) -> Result<NativeBlockDevice> {
        NativeBlockDevice::open_path(path)
    }

    fn from_file(file: File) -> NativeBlockDevice {
        NativeBlockDevice::from_file(file)
    }
}
