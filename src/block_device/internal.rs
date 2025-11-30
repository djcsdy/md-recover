use crate::block_device::{BlockNumber, BlockSize};
use derive_more::{AsMut, AsRef, From, Into};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, From, Into, AsRef, AsMut)]
pub struct InternalFile(pub File);

impl InternalFile {
    pub fn read_block(
        &mut self,
        block_number: BlockNumber,
        block_size: BlockSize,
        buf: &mut [u8],
    ) -> io::Result<usize> {
        if buf.len() < usize::from(block_size) {
            Err(io::ErrorKind::InvalidInput.into())
        } else {
            self.as_mut().seek(SeekFrom::Start(
                block_number
                    .byte_pos(block_size)
                    .ok_or(io::ErrorKind::InvalidData)?,
            ))?;
            self.as_mut()
                .read_exact(&mut buf[..usize::from(block_size)])?;
            Ok(usize::from(block_size))
        }
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self(self.0.try_clone()?))
    }
}
