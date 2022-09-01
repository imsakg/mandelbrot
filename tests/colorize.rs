use mandelbrot::{render, ColorSettings, RenderSettings, Rgb, Viewport};
#[test]
fn colors_preserve_black_interiors_and_smooth_exteriors() {
    let s = RenderSettings {
        width: 4,
        height: 2,
        viewport: Viewport::new(-0.01, 0.01, -0.01, 0.01).unwrap(),
        ..Default::default()
    };
    assert!(render(&s)
        .unwrap()
        .colorize(&ColorSettings::default())
        .unwrap()
        .iter()
        .all(|p| *p == Rgb(0, 0, 0)));
    let f = render(&RenderSettings {
        width: 20,
        height: 15,
        ..Default::default()
    })
    .unwrap();
    let smooth = f.colorize(&ColorSettings::default()).unwrap();
    let bands = f
        .colorize(&ColorSettings {
            smooth: false,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(smooth.len(), 300);
    assert_ne!(smooth, bands);
}
