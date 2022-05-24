use mandelbrot::escape_point;

#[test]
fn smooth_counts_are_finite_and_not_integer_bands() {
    let e = escape_point(3.0, 0.0, 100);
    assert!(e.smooth(2).is_finite());
    assert!((e.smooth(2) - e.iterations as f64).abs() > 0.01);
    assert_eq!(escape_point(0.0, 0.0, 100).smooth(2), 100.0);
    assert!(escape_point(1e300, 0.0, 10).smooth(2).is_finite());
    assert_eq!(e.smooth(0), e.iterations as f64);
}
