use crate::lvm::crc::LVM_CRC;
use crate::lvm::units::SectorNumber;
use crate::lvm::SECTOR_SIZE_BYTES;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct LvmLabel {
    sector_number: SectorNumber,
    label_type: [u8; 8],
    header: Vec<u8>,
}

impl LvmLabel {
    const MAGIC: &[u8; 8] = b"LABELONE";
    const SIZE_BYTES: usize = SECTOR_SIZE_BYTES;
    const SCAN_SECTORS: u64 = 4;

    pub fn scan_read<D>(device: &mut D) -> io::Result<Self>
    where
        D: Read + Seek,
    {
        for sector_number in (0..Self::SCAN_SECTORS).map(SectorNumber) {
            device.seek(SeekFrom::Start(sector_number.byte_pos().unwrap()))?;
            match Self::read(device) {
                Ok(label) if label.sector_number == sector_number => return Ok(label),
                Ok(_) => return Err(io::ErrorKind::InvalidData.into()),
                Err(error) if error.kind() == io::ErrorKind::InvalidData => continue,
                Err(error) => return Err(error),
            }
        }

        Err(io::ErrorKind::InvalidData.into())
    }

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut sector = [0u8; Self::SIZE_BYTES];
        reader.read_exact(&mut sector)?;

        let mut reader = &sector[..];

        let mut magic = [0u8; Self::MAGIC.len()];
        reader.read_exact(&mut magic)?;
        if &magic != Self::MAGIC {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let sector_number = reader.read_u64::<LittleEndian>()?.into();
        let crc = reader.read_u32::<LittleEndian>()?;

        if LVM_CRC.checksum(reader) != crc {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let header_offset = usize::try_from(reader.read_u32::<LittleEndian>()?).unwrap();

        if header_offset > sector.len() {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let mut label_type = [0u8; 8];
        reader.read_exact(&mut label_type)?;

        let header = sector[header_offset..].into();

        Ok(LvmLabel {
            sector_number,
            label_type,
            header,
        })
    }

    pub fn sector_number(&self) -> SectorNumber {
        self.sector_number
    }

    pub fn label_type(&self) -> &[u8; 8] {
        &self.label_type
    }

    pub fn header(&self) -> &Vec<u8> {
        &self.header
    }
}
