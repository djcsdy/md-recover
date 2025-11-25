use crc::{Algorithm, Crc, CRC_32_ISCSI};

pub const EXT4_CRC32C: Crc<u32> = Crc::<u32>::new(&Algorithm {
    xorout: 0,
    ..CRC_32_ISCSI
});

#[cfg(test)]
mod test {
    use crate::ext4::crc::EXT4_CRC32C;

    #[test]
    fn test() {
        let mut digest = EXT4_CRC32C.digest();
        digest.update(&[0x12, 0x34]);
        let a = digest.finalize();

        let mut digest = EXT4_CRC32C.digest();
        digest.update(&[0x12]);
        let b = digest.finalize();

        let mut digest = EXT4_CRC32C.digest_with_initial(b.reverse_bits());
        digest.update(&[0x34]);
        let c = digest.finalize();

        assert_eq!(a, c);
    }
}
