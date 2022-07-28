use mandelbrot::{render, RenderSettings};
#[test]
fn frame_accessors_never_wrap_out_of_range_coordinates() {
    let f = render(&RenderSettings {
        width: 3,
        height: 2,
        ..Default::default()
    })
    .unwrap();
    assert_eq!(f.row(0).unwrap(), &f.samples()[..3]);
    assert_eq!(f.at(2, 1), Some(&f.samples()[5]));
    assert!(f.row(2).is_none());
    assert!(f.at(3, 0).is_none());
    assert!(f.at(usize::MAX, 0).is_none());
}
