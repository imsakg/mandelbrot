use mandelbrot::{cli::Options, Fractal};
#[test]
fn variant_flags_select_julia_ship_and_multibrot_without_silent_fallback() {
    assert_eq!(
        Options::parse(["--julia", "-0.8,0.156"])
            .unwrap()
            .render
            .iterations
            .fractal,
        Fractal::Julia {
            re: -0.8,
            im: 0.156
        }
    );
    assert_eq!(
        Options::parse(["--variant", "ship"])
            .unwrap()
            .render
            .iterations
            .fractal,
        Fractal::BurningShip
    );
    assert_eq!(
        Options::parse(["--power", "5"])
            .unwrap()
            .render
            .iterations
            .fractal,
        Fractal::Multibrot { power: 5 }
    );
    for args in [
        ["--variant", "unknown"],
        ["--power", "9"],
        ["--julia", "0,NaN"],
        ["--radius", "1"],
    ] {
        assert!(Options::parse(args).is_err());
    }
}
