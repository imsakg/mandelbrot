use mandelbrot::{Image, ImageFormat, Rgb};
use std::path::Path;
#[test]
fn image_formats_use_explicit_supported_extensions() {
    assert_eq!(
        ImageFormat::from_path(Path::new("scene.PNG")).unwrap(),
        ImageFormat::Png
    );
    assert_eq!(
        ImageFormat::from_path(Path::new("scene.ppm")).unwrap(),
        ImageFormat::Ppm
    );
    assert!(ImageFormat::from_path(Path::new("scene.jpg")).is_err());
    assert!(ImageFormat::from_path(Path::new("scene")).is_err());
    let image = Image::new(1, 1, vec![Rgb(1, 2, 3)]).unwrap();
    let mut out = vec![];
    image.write(&mut out, ImageFormat::Png).unwrap();
    assert!(out.starts_with(&[137, 80, 78, 71]));
}
