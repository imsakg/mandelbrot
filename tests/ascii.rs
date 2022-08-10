use mandelbrot::{render, RenderSettings, Viewport};
#[test]
fn ascii_output_has_exact_dimensions_and_distinguishes_interior() {
    let s = RenderSettings {
        width: 7,
        height: 3,
        viewport: Viewport::new(-0.01, 0.01, -0.01, 0.01).unwrap(),
        ..Default::default()
    };
    let text = render(&s).unwrap().ascii();
    assert_eq!(
        text,
        "@@@@@@@
@@@@@@@
@@@@@@@
"
    );
    let exterior = render(&RenderSettings {
        width: 7,
        height: 3,
        ..Default::default()
    })
    .unwrap()
    .ascii();
    assert_eq!(exterior.lines().count(), 3);
    assert!(exterior.lines().all(|l| l.len() == 7));
}
