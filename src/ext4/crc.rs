use crc::{Algorithm, CRC_32_ISCSI};

pub const EXT4_CRC32C: Algorithm<u32> = Algorithm {
    xorout: 0,
    ..CRC_32_ISCSI
};
