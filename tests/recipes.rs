use mandelbrot::cli::Options;
#[test]
fn portable_render_recipes_round_trip_float_settings_and_colors() {
    let o = Options::parse([
        "--preset",
        "julia",
        "--zoom",
        "2",
        "--width",
        "31",
        "--gamma",
        "2.2",
        "--cycles",
        "3",
        "--histogram",
        "--threads",
        "2",
    ])
    .unwrap();
    let loaded = Options::parse(o.recipe().split_whitespace()).unwrap();
    assert_eq!(loaded, o);
    let o = Options::parse([
        "--bounds",
        "-0.743643887038,-0.743643887036,0.131825904204,0.131825904206",
    ])
    .unwrap();
    assert_eq!(Options::parse(o.recipe().split_whitespace()).unwrap(), o);
}
