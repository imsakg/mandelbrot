use mandelbrot::{escape_point, in_main_bulbs};

#[test]
fn cardioid_and_period_two_bulb_are_recognized_conservatively() {
    for (x, y) in [(0.0, 0.0), (-1.0, 0.0), (-0.125, 0.1)] {
        assert!(in_main_bulbs(x, y));
        assert!(!escape_point(x, y, 100).escaped);
    }
    for (x, y) in [(0.5, 0.5), (-2.0, 0.1), (f64::NAN, 0.0), (1e300, 1e300)] {
        assert!(!in_main_bulbs(x, y));
    }
}
