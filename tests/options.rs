use mandelbrot::cli::Options;
#[test]
fn basic_options_validate_sizes_and_iteration_limits() {
    let o = Options::parse(["--width", "40", "--height", "20", "--iterations", "800"]).unwrap();
    assert_eq!(
        (
            o.render.width,
            o.render.height,
            o.render.iterations.max_iterations
        ),
        (40, 20, 800)
    );
    assert!(Options::parse(["--width"]).is_err());
    assert!(Options::parse(["--width", "0"]).is_err());
    assert!(Options::parse(["--unknown"]).is_err());
    assert!(Options::parse(["--iterations", "0"]).is_err());
    assert!(Options::parse(["--help"]).unwrap().help);
}
