use mandelbrot::{iterate, Fractal, IterationSettings};
#[test]
fn overflowing_orbits_escape_instead_of_becoming_nan_interiors() {
    let s = IterationSettings {
        fractal: Fractal::Julia { re: 0.0, im: 0.0 },
        ..Default::default()
    };
    assert!(iterate(1e200, 1e200, &s).unwrap().escaped);
    for power in 5..=8 {
        let s = IterationSettings {
            fractal: Fractal::Multibrot { power },
            escape_radius: 1e150,
            ..Default::default()
        };
        let e = iterate(1e100, 0.0, &s).unwrap();
        assert!(e.escaped);
        assert!(!e.norm_sqr.is_nan());
        assert!(e.smooth(power).is_finite());
    }
}
