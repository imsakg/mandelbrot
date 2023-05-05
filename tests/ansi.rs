use mandelbrot::{cli::Options, render, ColorSettings, RenderSettings};
#[test]
fn truecolor_is_opt_in_and_every_row_restores_terminal_attributes() {
    assert!(Options::parse(["--color"]).unwrap().color);
    let f = render(&RenderSettings {
        width: 4,
        height: 3,
        ..Default::default()
    })
    .unwrap();
    let text = f.ansi(&ColorSettings::default()).unwrap();
    assert_eq!(text.lines().count(), 3);
    assert!(text.lines().all(|row| row.ends_with("\x1b[0m")));
    assert!(text.contains("\x1b[48;2;"));
    assert!(!f.ascii().contains('\x1b'));
}
