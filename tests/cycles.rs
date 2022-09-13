use mandelbrot::{render, ColorSettings, RenderSettings};
#[test]
fn palette_cycles_and_offsets_wrap_and_reject_nonfinite_values() {
    let f = render(&RenderSettings {
        width: 9,
        height: 7,
        ..Default::default()
    })
    .unwrap();
    let base = f.colorize(&ColorSettings::default()).unwrap();
    let shifted = f
        .colorize(&ColorSettings {
            shift: 0.3,
            cycles: 3.0,
            ..Default::default()
        })
        .unwrap();
    assert_ne!(base, shifted);
    assert_eq!(
        base,
        f.colorize(&ColorSettings {
            shift: 1.0,
            ..Default::default()
        })
        .unwrap()
    );
    for cycles in [0.0, -1.0, f64::NAN, 1e300] {
        assert!(f
            .colorize(&ColorSettings {
                cycles,
                ..Default::default()
            })
            .is_err());
    }
}
