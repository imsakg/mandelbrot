use mandelbrot::{render, ColorSettings, RenderSettings};
#[test]
fn gamma_is_validated_and_lifts_midtones_without_changing_interior() {
    let f = render(&RenderSettings {
        width: 12,
        height: 8,
        ..Default::default()
    })
    .unwrap();
    let normal = f.colorize(&ColorSettings::default()).unwrap();
    let bright = f
        .colorize(&ColorSettings {
            gamma: 2.2,
            ..Default::default()
        })
        .unwrap();
    assert_ne!(normal, bright);
    for (a, b) in normal.iter().zip(bright) {
        assert!(b.0 >= a.0 && b.1 >= a.1 && b.2 >= a.2);
    }
    for gamma in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(ColorSettings {
            gamma,
            ..Default::default()
        }
        .validate()
        .is_err());
    }
}
