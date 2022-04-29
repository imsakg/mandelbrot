use mandelbrot::{iterate, IterationSettings};

#[test]
fn larger_escape_radius_retains_the_orbit_longer() {
    let mut s = IterationSettings::default();
    s.escape_radius = 8.0;
    assert_eq!(iterate(3.0, 0.0, &s).unwrap().iterations, 2);
    for invalid in [0.0, 1.0, -3.0, f64::NAN, f64::INFINITY, 1e200] {
        s.escape_radius = invalid;
        assert!(s.validate().is_err());
    }
}
