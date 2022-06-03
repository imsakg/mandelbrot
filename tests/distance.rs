use mandelbrot::{escape_point, iterate, Fractal, IterationSettings};

#[test]
fn exterior_distance_is_positive_and_interior_has_no_estimate() {
    let e = escape_point(3.0, 0.0, 100);
    assert!((e.distance.unwrap() - 3.0 * 3.0_f64.ln()).abs() < 1e-12);
    assert_eq!(escape_point(0.0, 0.0, 100).distance, None);
    let s = IterationSettings {
        fractal: Fractal::BurningShip,
        ..Default::default()
    };
    assert_eq!(iterate(3.0, 0.0, &s).unwrap().distance, None);
}
