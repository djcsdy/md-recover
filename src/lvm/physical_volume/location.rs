use binary_layout::binary_layout;
use std::io;
use std::io::{Read, Seek, SeekFrom};

binary_layout!(layout, LittleEndian, {
    offset_bytes: u64,
    size_bytes: u64,
});

#[derive(Clone)]
pub struct DiskLocation<S: AsRef<[u8]>>(S);

impl<S: AsRef<[u8]>> DiskLocation<S> {
    pub fn new(storage: S) -> Self {
        Self(storage)
    }

    pub fn offset_bytes(&self) -> u64 {
        self.view().offset_bytes().read()
    }

    pub fn size_bytes(&self) -> u64 {
        self.view().size_bytes().read()
    }

    pub fn is_end_marker(&self) -> bool {
        self.offset_bytes() == 0 && self.size_bytes() == 0
    }

    fn view(&self) -> layout::View<&[u8]> {
        layout::View::new(self.0.as_ref())
    }
}

impl DiskLocation<Vec<u8>> {
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut buf = vec![0u8; layout::SIZE.unwrap()];
        reader.read_exact(&mut buf)?;
        Ok(Self::new(buf))
    }

    pub fn read_all<R: Read>(mut reader: R) -> io::Result<Vec<Self>> {
        let mut locations = Vec::with_capacity(1);
        loop {
            let location = Self::read(&mut reader)?;
            if location.is_end_marker() {
                return Ok(locations);
            } else {
                locations.push(location);
            }
        }
    }

    pub fn seek(&self, mut device: impl Read + Seek) -> io::Result<u64> {
        device.seek(SeekFrom::Start(self.offset_bytes()))
    }

    pub fn take<D: Read + Seek>(&self, mut device: D) -> io::Result<io::Take<D>> {
        self.seek(&mut device)?;
        Ok(device.take(self.size_bytes()))
    }
}
