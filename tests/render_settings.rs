use mandelbrot::RenderSettings;

#[test]
fn image_sizes_have_checked_area_and_resource_limits() {
    let mut s = RenderSettings::default();
    assert_eq!(s.validate().unwrap(), 800 * 600);
    for (w, h) in [(0, 1), (1, 0), (usize::MAX, 2), (100_000, 100_000)] {
        s.width = w;
        s.height = h;
        assert!(s.validate().is_err());
    }
}
