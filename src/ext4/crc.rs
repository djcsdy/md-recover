use crc::{Algorithm, Crc, CRC_32_ISCSI};

const EXT4_CRC32C: Crc<u32> = Crc::<u32>::new(&Algorithm {
    xorout: 0,
    ..CRC_32_ISCSI
});

pub const EXT4_CRC32C_INITIAL: u32 = EXT4_CRC32C.algorithm.init;

pub fn ext4_crc32c<B: AsRef<[u8]>>(initial: u32, bytes: B) -> u32 {
    let mut digest = EXT4_CRC32C.digest_with_initial(initial.reverse_bits());
    digest.update(bytes.as_ref());
    digest.finalize()
}

#[cfg(test)]
mod test {
    use crate::ext4::crc::{ext4_crc32c, EXT4_CRC32C_INITIAL};

    #[test]
    fn test() {
        assert_eq!(
            ext4_crc32c(EXT4_CRC32C_INITIAL, &[0x12, 0x34]),
            ext4_crc32c(ext4_crc32c(EXT4_CRC32C_INITIAL, &[0x12]), &[0x34])
        );
    }
}
