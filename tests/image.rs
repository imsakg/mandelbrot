use mandelbrot::{Image, Rgb};
#[test]
fn images_keep_dimensions_and_pixels_consistent() {
    let pixels = vec![Rgb(1, 2, 3), Rgb(4, 5, 6)];
    let image = Image::new(2, 1, pixels.clone()).unwrap();
    assert_eq!((image.width(), image.height()), (2, 1));
    assert_eq!(image.pixels(), pixels);
    assert!(Image::new(2, 2, pixels).is_err());
    assert!(Image::new(0, 0, vec![]).is_err());
    assert!(Image::new(usize::MAX, 2, vec![]).is_err());
}
