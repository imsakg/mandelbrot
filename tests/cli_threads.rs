use mandelbrot::{cli::Options, render_rgb, render_rgb_parallel, ColorSettings, RenderSettings};
#[test]
fn worker_option_is_bounded_and_parallel_supersampling_matches_serial() {
    assert_eq!(Options::parse(["--threads", "3"]).unwrap().threads, 3);
    assert!(Options::parse(["--threads", "0"]).is_err());
    assert!(Options::parse(["--threads", "65"]).is_err());
    let s = RenderSettings {
        width: 13,
        height: 9,
        ..Default::default()
    };
    let c = ColorSettings::default();
    assert_eq!(
        render_rgb(&s, &c, 2).unwrap(),
        render_rgb_parallel(&s, &c, 2, 3).unwrap()
    );
}
