use mandelbrot::Viewport;

#[test]
fn centered_zoom_is_reversible_and_keeps_the_center() {
    let v = Viewport::default();
    let zoom = v.zoom(2.0).unwrap();
    assert_eq!(zoom.coordinate(0.5, 0.5), v.coordinate(0.5, 0.5));
    assert_eq!(zoom.spans(), (1.5, 1.0));
    assert_eq!(zoom.zoom(0.5).unwrap(), v);
    for bad in [0.0, -1.0, f64::INFINITY, f64::NAN] {
        assert!(v.zoom(bad).is_err());
    }
}
