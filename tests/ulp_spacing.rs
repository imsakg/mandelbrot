use mandelbrot::Viewport;
#[test]
fn resolution_validation_checks_actual_adjacent_centers_not_only_bounds() {
    let v = Viewport::new(1.0, 1.0 + 3.0 * f64::EPSILON, -1.0, 1.0).unwrap();
    assert_eq!(
        v.pixel_center(2, 0, 5, 1).unwrap().0,
        v.pixel_center(3, 0, 5, 1).unwrap().0
    );
    assert!(v.validate_resolution(5, 1).is_err());
}
