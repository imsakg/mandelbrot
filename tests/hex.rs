use mandelbrot::{cli::Options, Rgb};
#[test]
fn hexadecimal_interior_colors_are_exact_and_round_trip_in_recipes() {
    assert_eq!(Rgb::from_hex("#0a14ff").unwrap(), Rgb(10, 20, 255));
    assert_eq!(Rgb(10, 20, 255).hex(), "#0a14ff");
    for invalid in ["fff", "#00000g", "#１２３４５６", "#1234567"] {
        assert!(Rgb::from_hex(invalid).is_err());
    }
    let o = Options::parse(["--interior", "#0a14ff"]).unwrap();
    assert_eq!(o.colors.interior, Rgb(10, 20, 255));
    assert_eq!(Options::parse(o.recipe().split_whitespace()).unwrap(), o);
}
