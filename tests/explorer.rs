use mandelbrot::{cli::Options, controls::Action, Explorer, Palette};
#[test]
fn explorer_unifies_undo_redo_palette_and_reset_actions() {
    let mut e = Explorer::new(Options::parse([] as [&str; 0]).unwrap()).unwrap();
    let original = e.options().render.viewport;
    e.key('k').unwrap();
    let zoom = e.options().render.viewport;
    e.key('u').unwrap();
    assert_eq!(e.options().render.viewport, original);
    e.key('y').unwrap();
    assert_eq!(e.options().render.viewport, zoom);
    e.key('p').unwrap();
    assert_eq!(e.options().colors.palette, Palette::Fire);
    e.key('r').unwrap();
    assert_eq!(e.options().render.viewport, original);
    assert_eq!(e.key('q').unwrap(), Action::Quit);
}
