use mandelbrot::{cli::Options, presets, render};
#[test]
fn named_views_validate_and_render_with_cli_override_order() {
    for preset in presets::ALL {
        let o =
            Options::parse(["--preset", preset.name, "--width", "20", "--height", "12"]).unwrap();
        assert_eq!(render(&o.render).unwrap().samples().len(), 240);
    }
    assert_eq!(
        Options::parse(["--preset", "seahorse", "--iterations", "90"])
            .unwrap()
            .render
            .iterations
            .max_iterations,
        90
    );
    assert!(Options::parse(["--preset", "missing"]).is_err());
}
