use mandelbrot::{render, ColorSettings, RenderSettings, Rgb};
#[test]
fn chosen_interior_color_is_used_without_gamma_shifts() {
    let f = render(&RenderSettings {
        width: 1,
        height: 1,
        ..Default::default()
    })
    .unwrap();
    let c = ColorSettings {
        interior: Rgb(10, 20, 30),
        gamma: 2.0,
        ..Default::default()
    };
    assert_eq!(f.colorize(&c).unwrap(), vec![Rgb(10, 20, 30)]);
}
