use mandelbrot::Viewport;

#[test]
fn fitting_expands_bounds_without_cropping_the_original_view() {
    let v = Viewport::default();
    let wide = v.fit_aspect(2.0).unwrap();
    assert_eq!(wide.spans(), (4.0, 2.0));
    assert_eq!(wide.coordinate(0.5, 0.5), (-0.5, 0.0));
    let tall = v.fit_aspect(1.0).unwrap();
    assert_eq!(tall.spans(), (3.0, 3.0));
    assert!(v.fit_aspect(0.0).is_err());
    assert!(v.fit_aspect(f64::NAN).is_err());
}
