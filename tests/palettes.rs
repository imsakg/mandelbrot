use mandelbrot::{Palette, Rgb};
#[test]
fn named_palettes_are_distinct_and_reproducible() {
    assert_eq!(Palette::Gray.sample(0.0), Rgb(0, 0, 0));
    assert_eq!(Palette::Gray.sample(1.0), Rgb(255, 255, 255));
    assert_ne!(Palette::Fire.sample(0.5), Palette::Ocean.sample(0.5));
    for p in Palette::ALL {
        assert_eq!(Palette::parse(p.name()).unwrap(), p);
        assert_eq!(p.sample(-10.0), p.sample(0.0));
    }
    assert!(Palette::parse("not-a-palette").is_err());
}
