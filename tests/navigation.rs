use mandelbrot::{Navigation, Viewport};

#[test]
fn navigation_supports_undo_redo_branching_and_bounded_history() {
    let v = Viewport::default();
    let a = v.zoom(2.0).unwrap();
    let b = a.pan(0.1, 0.0).unwrap();
    let mut n = Navigation::new(v, 2).unwrap();
    n.push(a);
    n.push(b);
    assert_eq!(n.undo(), Some(a));
    assert_eq!(n.undo(), None);
    assert_eq!(n.redo(), Some(b));
    n.undo();
    n.push(v);
    assert_eq!(n.redo(), None);
    assert_eq!(n.current(), v);
    assert!(Navigation::new(v, 0).is_err());
}
