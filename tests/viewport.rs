use mandelbrot::Viewport;

#[test]
fn viewport_requires_ordered_finite_nonzero_spans() {
    assert!(Viewport::new(-2.0, 1.0, -1.0, 1.0).is_ok());
    for bounds in [
        [1.0, 1.0, -1.0, 1.0],
        [2.0, 1.0, -1.0, 1.0],
        [-2.0, 1.0, f64::NAN, 1.0],
        [-1e308, 1e308, -1.0, 1.0],
    ] {
        assert!(Viewport::new(bounds[0], bounds[1], bounds[2], bounds[3]).is_err());
    }
}

#[test]
fn normalized_coordinates_cover_corners_and_center() {
    let v = Viewport::default();
    assert_eq!(v.coordinate(0.0, 0.0), (-2.0, -1.0));
    assert_eq!(v.coordinate(1.0, 1.0), (1.0, 1.0));
    assert_eq!(v.coordinate(0.5, 0.5), (-0.5, 0.0));
}
