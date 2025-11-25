use crc::{Algorithm, Crc, CRC_32_ISCSI};

pub const EXT4_CRC32C: Crc<u32> = Crc::<u32>::new(&Algorithm {
    xorout: 0,
    ..CRC_32_ISCSI
});
