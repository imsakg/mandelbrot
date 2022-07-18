use mandelbrot::Viewport;

#[test]
fn precision_guard_rejects_indistinguishable_adjacent_pixels() {
    assert!(Viewport::default().validate_resolution(800, 600).is_ok());
    let v = Viewport::new(1e16, 1e16 + 4.0, -1.0, 1.0).unwrap();
    assert!(v.validate_resolution(800, 600).is_err());
    assert!(Viewport::default().validate_resolution(0, 1).is_err());
    assert!(Viewport::default().zoom_at(1e300, 0.3, 0.7).is_err());
}
