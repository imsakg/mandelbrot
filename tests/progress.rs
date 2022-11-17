use mandelbrot::{render, render_with_progress, RenderSettings};
#[test]
fn progress_is_monotone_and_cancellation_returns_no_partial_frame() {
    let s = RenderSettings {
        width: 7,
        height: 5,
        ..Default::default()
    };
    let mut seen = vec![];
    let frame = render_with_progress(&s, |done, total| {
        seen.push((done, total));
        true
    })
    .unwrap();
    assert_eq!(frame, render(&s).unwrap());
    assert_eq!(seen, vec![(1, 5), (2, 5), (3, 5), (4, 5), (5, 5)]);
    let mut visits = 0;
    assert!(render_with_progress(&s, |_, _| {
        visits += 1;
        visits < 2
    })
    .is_err());
    assert_eq!(visits, 2);
}
