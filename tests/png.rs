use mandelbrot::{Image, Rgb};
#[test]
fn png_contains_ihdr_idat_and_iend_with_rgb_dimensions() {
    let image = Image::new(2, 1, vec![Rgb(1, 2, 3), Rgb(255, 0, 128)]).unwrap();
    let mut data = vec![];
    image.write_png(&mut data).unwrap();
    assert_eq!(&data[..8], b"\x89PNG\r\n\x1a\n");
    assert_eq!(&data[12..16], b"IHDR");
    assert_eq!(&data[16..24], &[0, 0, 0, 2, 0, 0, 0, 1]);
    assert_eq!(&data[24..29], &[8, 2, 0, 0, 0]);
    let mut at = 8;
    let mut names = vec![];
    while at < data.len() {
        let n = u32::from_be_bytes(data[at..at + 4].try_into().unwrap()) as usize;
        names.push(data[at + 4..at + 8].to_vec());
        at += n + 12;
    }
    assert_eq!(
        names,
        vec![b"IHDR".to_vec(), b"IDAT".to_vec(), b"IEND".to_vec()]
    );
    assert_eq!(at, data.len());
}
