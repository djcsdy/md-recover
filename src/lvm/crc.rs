use crc::{Algorithm, Crc, CRC_32_ISCSI};

pub const LVM_CRC: Crc<u32> = Crc::<u32>::new(&Algorithm {
    xorout: 0,
    init: 0xf597_a6cf_u32.reverse_bits(),
    ..CRC_32_ISCSI
});
