use mandelbrot::{iterate, Fractal, IterationSettings};

#[test]
fn julia_uses_the_sample_as_z_and_a_fixed_parameter() {
    let s = IterationSettings {
        fractal: Fractal::Julia { re: 0.0, im: 0.0 },
        ..Default::default()
    };
    assert!(!iterate(0.5, 0.0, &s).unwrap().escaped);
    assert_eq!(iterate(1.5, 0.0, &s).unwrap().iterations, 1);
    assert!(iterate(3.0, 0.0, &s).unwrap().escaped);
}

#[test]
fn julia_constants_must_be_finite() {
    let s = IterationSettings {
        fractal: Fractal::Julia {
            re: f64::NAN,
            im: 0.0,
        },
        ..Default::default()
    };
    assert!(s.validate().is_err());
}
