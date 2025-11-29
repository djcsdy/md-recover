use crate::block_device::{BlockCount, BlockDevice, BlockNumber, BlockSize};
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Clone, Debug)]
pub struct InMemoryBlockDevice {
    mem: Vec<u8>,
    pos: usize,
    block_size: BlockSize,
}

impl InMemoryBlockDevice {
    pub fn new(mem: impl Into<Vec<u8>>, block_size: BlockSize) -> Self {
        Self {
            mem: mem.into(),
            pos: 0,
            block_size,
        }
    }
}

impl BlockDevice for InMemoryBlockDevice {
    fn block_size(&self) -> io::Result<BlockSize> {
        Ok(self.block_size)
    }

    fn block_count(&self) -> io::Result<BlockCount> {
        Ok(BlockCount(
            u64::try_from(self.mem.len()).or(Err(io::Error::from(io::ErrorKind::InvalidData)))?
                / u64::from(self.block_size),
        ))
    }

    fn read_block(&mut self, block_number: BlockNumber, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() < usize::from(self.block_size) {
            Err(io::ErrorKind::InvalidInput.into())
        } else {
            let offset = usize::try_from(block_number.byte_pos(self.block_size))
                .or(Err(io::Error::from(io::ErrorKind::InvalidInput)))?;
            buf.copy_from_slice(&self.mem[offset..][..usize::from(self.block_size)]);
            Ok(usize::from(self.block_size))
        }
    }

    fn try_clone(&self) -> io::Result<Self> {
        Ok(self.clone())
    }
}

impl Read for InMemoryBlockDevice {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let available = self.mem.len().saturating_sub(self.pos);
        let to_read = buf.len().clamp(0, available);
        buf[..to_read].copy_from_slice(&self.mem[self.pos..][..to_read]);
        self.pos += to_read;
        Ok(to_read)
    }
}

impl Seek for InMemoryBlockDevice {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
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
