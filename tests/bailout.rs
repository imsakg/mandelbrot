use mandelbrot::escape_point;

#[test]
fn last_update_can_escape_without_being_mistaken_for_interior() {
    let e = escape_point(3.0, 0.0, 1);
    assert!(e.escaped);
    assert_eq!(e.iterations, 1);
}

#[test]
fn bailout_circle_is_strict_and_orbits_are_conjugate() {
    assert!(!escape_point(2.0, 0.0, 1).escaped);
    for re in [-2.0, -0.7, 0.1, 0.4] {
        assert_eq!(escape_point(re, 0.6, 80), escape_point(re, -0.6, 80));
    }
}
