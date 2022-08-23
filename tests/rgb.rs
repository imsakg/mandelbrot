use mandelbrot::Rgb;
#[test]
fn rgb_interpolation_clamps_endpoints_and_rounds_channels() {
    let a = Rgb(0, 20, 100);
    let b = Rgb(255, 120, 200);
    assert_eq!(a.lerp(b, 0.0), a);
    assert_eq!(a.lerp(b, 1.0), b);
    assert_eq!(a.lerp(b, 0.5), Rgb(128, 70, 150));
    assert_eq!(a.lerp(b, -1.0), a);
    assert_eq!(a.lerp(b, f64::NAN), a);
    assert_eq!(b.bytes(), [255, 120, 200]);
}
