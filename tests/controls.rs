use mandelbrot::{
    controls::{apply, Action},
    RenderSettings,
};
#[test]
fn navigation_keys_transform_views_and_keep_iteration_budgets_bounded() {
    let mut s = RenderSettings {
        width: 40,
        height: 20,
        ..Default::default()
    };
    let original = s.clone();
    assert_eq!(apply(&mut s, 'k').unwrap(), Action::Redraw);
    assert_eq!(s.viewport.spans(), (1.5, 1.0));
    apply(&mut s, 'j').unwrap();
    assert_eq!(s.viewport, original.viewport);
    apply(&mut s, 'd').unwrap();
    assert_ne!(s.viewport, original.viewport);
    apply(&mut s, 'r').unwrap();
    assert_eq!(s.viewport, original.viewport);
    for _ in 0..40 {
        apply(&mut s, ']').unwrap();
    }
    assert_eq!(s.iterations.max_iterations, 10_000_000);
    for _ in 0..40 {
        apply(&mut s, '[').unwrap();
    }
    assert_eq!(s.iterations.max_iterations, 1);
    assert_eq!(apply(&mut s, 'q').unwrap(), Action::Quit);
    assert_eq!(apply(&mut s, '\u{1b}').unwrap(), Action::Quit);
    assert_eq!(apply(&mut s, '\u{3}').unwrap(), Action::Quit);
    assert_eq!(apply(&mut s, '?').unwrap(), Action::None);
}
