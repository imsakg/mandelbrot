use mandelbrot::{Image, Rgb};
#[test]
fn stored_deflate_crosses_block_boundaries_without_dropping_scanline_bytes() {
    let image = Image::new(128, 171, vec![Rgb(7, 8, 9); 128 * 171]).unwrap();
    let mut data = vec![];
    image.write_png(&mut data).unwrap();
    let mut at = 8;
    let mut compressed = vec![];
    while at < data.len() {
        let n = u32::from_be_bytes(data[at..at + 4].try_into().unwrap()) as usize;
        if &data[at + 4..at + 8] == b"IDAT" {
            compressed.extend_from_slice(&data[at + 8..at + 8 + n]);
        }
        at += n + 12;
    }
    let mut cursor = 2;
    let mut decoded = vec![];
    let mut blocks = 0;
    loop {
        let flag = compressed[cursor];
        cursor += 1;
        assert!(flag == 0 || flag == 1);
        let len = u16::from_le_bytes(compressed[cursor..cursor + 2].try_into().unwrap());
        let inv = u16::from_le_bytes(compressed[cursor + 2..cursor + 4].try_into().unwrap());
        assert_eq!(len, !inv);
        cursor += 4;
        decoded.extend_from_slice(&compressed[cursor..cursor + len as usize]);
        cursor += len as usize;
        blocks += 1;
        if flag == 1 {
            break;
        }
    }
    assert_eq!(blocks, 2);
    assert_eq!(cursor + 4, compressed.len());
    assert_eq!(decoded.len(), 171 * (128 * 3 + 1));
    for row in decoded.as_chunks::<{ 128 * 3 + 1 }>().0 {
        assert_eq!(row[0], 0);
        assert!(row[1..].as_chunks::<3>().0.iter().all(|p| *p == [7, 8, 9]));
    }
}
