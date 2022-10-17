pub(crate) fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in bytes {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320u32.wrapping_mul(crc & 1));
        }
    }
    !crc
}
pub(crate) fn adler32(bytes: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &byte in bytes {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

#[cfg(test)]
mod tests {
    #[test]
    fn standard_checksum_vectors_and_empty_inputs() {
        assert_eq!(super::crc32(b"123456789"), 3421780262);
        assert_eq!(super::adler32(b"123456789"), 152961502);
        assert_eq!(super::crc32(b""), 0);
        assert_eq!(super::adler32(b""), 1);
    }
}
