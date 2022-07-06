use mandelbrot::Viewport;

#[test]
fn pan_speed_scales_with_the_visible_span() {
    let v = Viewport::default();
    let p = v.pan(0.25, -0.5).unwrap();
    assert_eq!(p.spans(), v.spans());
    assert_eq!(p.coordinate(0.5, 0.5), (0.25, -1.0));
    assert_eq!(p.pan(-0.25, 0.5).unwrap(), v);
    assert!(v.pan(f64::INFINITY, 0.0).is_err());
}
