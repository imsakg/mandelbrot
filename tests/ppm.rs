use mandelbrot::{Image, Rgb};
#[test]
fn binary_ppm_contains_rgb_bytes_and_a_correct_header() {
    let image = Image::new(2, 1, vec![Rgb(1, 2, 3), Rgb(255, 0, 128)]).unwrap();
    let mut data = vec![];
    image.write_ppm(&mut data).unwrap();
    assert_eq!(data, b"P6\n2 1\n255\n\x01\x02\x03\xff\x00\x80");
}
