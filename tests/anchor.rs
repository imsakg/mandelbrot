use mandelbrot::Viewport;

#[test]
fn zoom_preserves_an_arbitrary_image_anchor() {
    let v = Viewport::default();
    let p = v.coordinate(0.25, 0.75);
    let z = v.zoom_at(3.0, 0.25, 0.75).unwrap();
    let got = z.coordinate(0.25, 0.75);
    assert!((got.0 - p.0).abs() < 1e-14 && (got.1 - p.1).abs() < 1e-14);
    assert!(v.zoom_at(2.0, -0.1, 0.5).is_err());
    assert!(v.zoom_at(2.0, 0.5, f64::NAN).is_err());
}
