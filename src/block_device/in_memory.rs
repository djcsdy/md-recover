use crate::block_device::BlockDevice;
use std::io::{Read, Seek, SeekFrom};

#[derive(Clone, Debug)]
pub struct InMemoryBlockDevice {
    mem: Vec<u8>,
    pos: usize,
}

impl InMemoryBlockDevice {
    pub fn new(mem: impl Into<Vec<u8>>) -> Self {
        Self {
            mem: mem.into(),
            pos: 0,
        }
    }
}

impl BlockDevice for InMemoryBlockDevice {
    fn size(&self) -> std::io::Result<u64> {
        Ok(self.mem.len().try_into().unwrap())
    }
}

impl Read for InMemoryBlockDevice {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let available = self.mem.len().saturating_sub(self.pos);
        let to_read = buf.len().clamp(0, available);
        buf[..to_read].copy_from_slice(&self.mem[self.pos..self.pos + to_read]);
        self.pos += to_read;
        Ok(to_read)
    }
}

impl Seek for InMemoryBlockDevice {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = match pos {
            SeekFrom::Start(offset) => offset.try_into().unwrap(),
            SeekFrom::End(offset) => self
                .mem
                .len()
                .saturating_add_signed(offset.try_into().unwrap()),
            SeekFrom::Current(offset) => self.pos.saturating_add_signed(offset.try_into().unwrap()),
        }
        .clamp(0, self.mem.len());

        Ok(self.pos.try_into().unwrap())
    }
}
