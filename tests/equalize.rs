use mandelbrot::{render, ColorSettings, RenderSettings};
#[test]
fn escaped_only_histogram_equalization_changes_contrast_without_division_by_zero() {
    let f = render(&RenderSettings {
        width: 30,
        height: 20,
        ..Default::default()
    })
    .unwrap();
    let c = ColorSettings {
        histogram: true,
        ..Default::default()
    };
    assert_ne!(
        f.colorize(&c).unwrap(),
        f.colorize(&ColorSettings::default()).unwrap()
    );
    let f = render(&RenderSettings {
        width: 1,
        height: 1,
        ..Default::default()
    })
    .unwrap();
    assert!(f.colorize(&c).is_ok());
}
