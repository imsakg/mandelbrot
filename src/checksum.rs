const fn crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = i as u32;
        let mut bit = 0;
        while bit < 8 {
            c = (c >> 1) ^ (0xedb8_8320u32.wrapping_mul(c & 1));
            bit += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
}
const CRC_TABLE: [u32; 256] = crc_table();
pub(crate) fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in bytes {
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ byte) as usize];
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
