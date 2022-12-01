use mandelbrot::{iterate, Fractal, IterationSettings};
#[test]
fn julia_bailout_adapts_to_large_constants_instead_of_losing_fixed_points() {
    let s = IterationSettings {
        fractal: Fractal::Julia { re: -6.0, im: 0.0 },
        ..Default::default()
    };
    let point = iterate(3.0, 0.0, &s).unwrap();
    assert!(!point.escaped);
    assert_eq!(point.iterations, s.max_iterations);
    let s = IterationSettings {
        fractal: Fractal::Julia { re: 1e300, im: 0.0 },
        ..Default::default()
    };
    assert!(s.validate().is_err());
}
