use mandelbrot::{render, render_rgb, ColorSettings, RenderSettings};
#[test]
fn supersampling_averages_a_regular_grid_and_respects_resource_limits() {
    let s = RenderSettings {
        width: 16,
        height: 12,
        ..Default::default()
    };
    let c = ColorSettings::default();
    assert_eq!(
        render_rgb(&s, &c, 1).unwrap(),
        render(&s).unwrap().colorize(&c).unwrap()
    );
    let aa = render_rgb(&s, &c, 2).unwrap();
    assert_eq!(aa.len(), 192);
    assert_eq!(aa, render_rgb(&s, &c, 2).unwrap());
    assert_ne!(aa, render_rgb(&s, &c, 1).unwrap());
    assert!(render_rgb(&s, &c, 0).is_err());
    assert!(render_rgb(&s, &c, 5).is_err());
    assert!(render_rgb(
        &RenderSettings {
            width: 2000,
            height: 2000,
            ..Default::default()
        },
        &c,
        4
    )
    .is_err());
}
