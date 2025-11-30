use crate::block_device::{BlockCount, BlockDevice, BlockNumber, BlockSize};
use derive_more::Display;
use num_integer::Integer;
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Display)]
#[display("{}", device)]
pub struct BlockDeviceReader<D: BlockDevice> {
    device: D,
    block_pos: BlockNumber,
    byte_pos: usize,
}

impl<D: BlockDevice> BlockDeviceReader<D> {
    pub fn new(device: D) -> Self {
        Self {
            device,
            block_pos: BlockNumber(0),
            byte_pos: 0,
        }
    }
}

impl<D: BlockDevice> BlockDevice for BlockDeviceReader<D> {
    fn block_size(&self) -> io::Result<BlockSize> {
        self.device.block_size()
    }

    fn block_count(&self) -> io::Result<BlockCount> {
        self.device.block_count()
    }

    fn read_block(&mut self, block_number: BlockNumber, buf: &mut [u8]) -> io::Result<usize> {
        self.device.read_block(block_number, buf)
    }

    fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            device: self.device.try_clone()?,
            block_pos: self.block_pos,
            byte_pos: self.byte_pos,
        })
    }
}

impl<D: BlockDevice> Read for BlockDeviceReader<D> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.block_pos >= self.device.block_count()? {
            return Ok(0);
        }

        let block_size = usize::from(self.device.block_size()?);
        let mut block = vec![0; block_size];
        self.device.read_block(self.block_pos, &mut block)?;
        let bytes_to_copy = buf.len().clamp(0, block_size - self.byte_pos);
        buf[..bytes_to_copy].copy_from_slice(&block[self.byte_pos..][..bytes_to_copy]);
        self.byte_pos += bytes_to_copy;
        if self.byte_pos >= block_size {
            self.byte_pos = 0;
            self.block_pos += BlockCount(1);
        }
        Ok(bytes_to_copy)
    }
}

impl<D: BlockDevice> Seek for BlockDeviceReader<D> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::End(offset) => self
                .device
                .block_count()?
                .size_bytes(self.device.block_size()?)
                .ok_or(io::ErrorKind::InvalidData)?
                .checked_add_signed(offset)
                .ok_or(io::ErrorKind::InvalidInput)?,
            SeekFrom::Current(offset) => self
                .block_pos
                .byte_pos(self.device.block_size()?)
                .ok_or(io::ErrorKind::InvalidData)?
                .checked_add(
                    u64::try_from(self.byte_pos)
                        .or(Err(io::Error::from(io::ErrorKind::InvalidData)))?,
                )
                .ok_or(io::ErrorKind::InvalidData)?
                .checked_add_signed(offset)
                .ok_or(io::ErrorKind::InvalidInput)?,
        };

        let (block_pos, byte_pos) = new_pos.div_rem(&u64::from(self.device.block_size()?));
        self.block_pos = BlockNumber(block_pos);
        self.byte_pos =
            usize::try_from(byte_pos).or(Err(io::Error::from(io::ErrorKind::InvalidInput)))?;
        Ok(new_pos)
    }
}

#[cfg(test)]
mod test {
    use crate::block_device::{BlockDeviceReader, BlockSize, InMemoryBlockDevice};
    use itertools::Itertools;
    use std::io::{Read, Seek, SeekFrom};

    fn reader() -> BlockDeviceReader<InMemoryBlockDevice> {
        BlockDeviceReader::new(InMemoryBlockDevice::new(
            (0..=255).into_iter().collect_vec(),
            BlockSize(32),
        ))
    }

    #[test]
    fn read_bytes() -> anyhow::Result<()> {
        let mut reader = reader();
        let mut buf = [0; 16];
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        Ok(())
    }

    #[test]
    fn read_bytes_sequentially() -> anyhow::Result<()> {
        let mut reader = reader();
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [0, 1, 2, 3]);
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [4, 5, 6, 7]);
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [8, 9, 10, 11]);
        Ok(())
    }

    #[test]
    fn read_bytes_across_block_boundary() -> anyhow::Result<()> {
        let mut reader = reader();
        let mut buf = [0; 48];
        reader.read_exact(&mut buf)?;
        assert_eq!(
            buf,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                44, 45, 46, 47
            ]
        );
        Ok(())
    }

    #[test]
    fn read_bytes_sequentially_across_block_boundary() -> anyhow::Result<()> {
        let mut reader = reader();
        let mut buf = [0; 24];
        reader.read_exact(&mut buf)?;
        assert_eq!(
            buf,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23
            ]
        );
        reader.read_exact(&mut buf)?;
        assert_eq!(
            buf,
            [
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
                45, 46, 47
            ]
        );
        Ok(())
    }

    #[test]
    fn seek_from_start_and_read_bytes() -> anyhow::Result<()> {
        let mut reader = reader();
        let mut buf = [0; 4];
        reader.seek(SeekFrom::Start(8))?;
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [8, 9, 10, 11]);
        Ok(())
    }

    #[test]
    fn seek_from_start_and_read_bytes_across_block_boundary() -> anyhow::Result<()> {
        let mut reader = reader();
        let mut buf = [0; 8];
        reader.seek(SeekFrom::Start(12))?;
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [12, 13, 14, 15, 16, 17, 18, 19]);
        Ok(())
    }

    #[test]
    fn read_bytes_and_seek_from_current_and_read_bytes_across_block_boundary() -> anyhow::Result<()>
    {
        let mut reader = reader();
        let mut buf = [0; 8];
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7]);
        reader.seek(SeekFrom::Current(6))?;
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [14, 15, 16, 17, 18, 19, 20, 21]);
        Ok(())
    }

    #[test]
    fn seek_from_end_and_read_bytes() -> anyhow::Result<()> {
        let mut reader = reader();
        let mut buf = [0; 4];
        reader.seek(SeekFrom::End(-4))?;
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [252, 253, 254, 255]);
        Ok(())
    }
}
