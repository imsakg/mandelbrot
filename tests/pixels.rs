use mandelbrot::Viewport;

#[test]
fn samples_are_at_pixel_centers_even_for_a_single_pixel() {
    let v = Viewport::default();
    assert_eq!(v.pixel_center(0, 0, 1, 1).unwrap(), (-0.5, 0.0));
    assert_eq!(v.pixel_center(0, 0, 2, 2).unwrap(), (-1.25, -0.5));
    assert!(v.pixel_center(2, 0, 2, 2).is_err());
    assert!(v.pixel_center(0, 0, 0, 2).is_err());
}
