use mandelbrot::{cli::Options, Palette};
#[test]
fn cli_color_settings_are_typed_and_checked() {
    let o = Options::parse([
        "--palette",
        "fire",
        "--cycles",
        "4",
        "--shift",
        "-0.2",
        "--gamma",
        "2.2",
        "--histogram",
        "--no-smooth",
        "--samples",
        "2",
    ])
    .unwrap();
    assert_eq!(o.colors.palette, Palette::Fire);
    assert_eq!(o.colors.gamma, 2.2);
    assert!(o.colors.histogram);
    assert!(!o.colors.smooth);
    assert_eq!(o.samples, 2);
    for args in [
        ["--palette", "bogus"],
        ["--samples", "5"],
        ["--gamma", "0"],
        ["--cycles", "0"],
    ] {
        assert!(Options::parse(args).is_err());
    }
}
