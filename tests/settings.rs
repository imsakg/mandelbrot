use mandelbrot::{iterate, IterationSettings};

#[test]
fn settings_reject_zero_iterations_and_nonfinite_coordinates() {
    let settings = IterationSettings {
        max_iterations: 0,
        ..Default::default()
    };
    assert!(iterate(0.0, 0.0, &settings).is_err());
    assert!(iterate(f64::NAN, 0.0, &IterationSettings::default()).is_err());
    assert!(iterate(0.0, f64::INFINITY, &IterationSettings::default()).is_err());
}

#[test]
fn default_settings_match_the_simple_kernel() {
    let settings = IterationSettings::default();
    assert_eq!(
        iterate(0.7, 0.1, &settings).unwrap(),
        mandelbrot::escape_point(0.7, 0.1, settings.max_iterations)
    );
}
