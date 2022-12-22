use mandelbrot::cli::Options;
#[test]
fn viewport_flags_apply_in_order_and_reject_invalid_geometry() {
    let o = Options::parse(["--bounds", "-2,2,-1,1", "--zoom", "2", "--pan", "0.25,0"]).unwrap();
    assert_eq!(o.render.viewport.spans(), (2.0, 1.0));
    assert_eq!(o.render.viewport.coordinate(0.5, 0.5), (0.5, 0.0));
    for args in [
        ["--bounds", "1,1,-1,1"],
        ["--bounds", "-2,2"],
        ["--zoom", "nan"],
        ["--pan", "1,nan"],
    ] {
        assert!(Options::parse(args).is_err());
    }
}
