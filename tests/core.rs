use mandelbrot::escape_point;

#[test]
fn origin_and_minus_one_stay_bounded() {
    assert!(!escape_point(0.0, 0.0, 100).escaped);
    assert!(!escape_point(-1.0, 0.0, 100).escaped);
}

#[test]
fn escape_counts_updates_not_loop_visits() {
    let point = escape_point(3.0, 0.0, 100);
    assert!(point.escaped);
    assert_eq!(point.iterations, 1);
    assert_eq!(escape_point(2.0, 0.0, 100).iterations, 2);
}
