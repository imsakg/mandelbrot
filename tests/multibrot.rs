use mandelbrot::{iterate, Fractal, IterationSettings};

#[test]
fn multibrot_power_two_agrees_with_mandelbrot() {
    let s = IterationSettings {
        fractal: Fractal::Multibrot { power: 2 },
        ..Default::default()
    };
    for (x, y) in [(-1.5, 0.3), (0.0, 0.0), (0.8, 0.2)] {
        assert_eq!(
            iterate(x, y, &s).unwrap(),
            iterate(x, y, &IterationSettings::default()).unwrap()
        );
    }
}

#[test]
fn cubic_and_invalid_powers_have_defined_behavior() {
    let mut s = IterationSettings {
        fractal: Fractal::Multibrot { power: 3 },
        ..Default::default()
    };
    assert!(!iterate(0.0, 0.0, &s).unwrap().escaped);
    assert!(iterate(1.1, 0.0, &s).unwrap().escaped);
    for power in [0, 1, 9, u32::MAX] {
        s.fractal = Fractal::Multibrot { power };
        assert!(s.validate().is_err());
    }
}
