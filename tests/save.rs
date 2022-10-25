use mandelbrot::{Image, ImageFormat, Rgb};
#[test]
fn saving_flushes_complete_images_and_never_clobbers_existing_files() {
    let path = std::env::temp_dir().join(format!("mandelbrot-save-{}.png", std::process::id()));
    let image = Image::new(1, 1, vec![Rgb(10, 20, 30)]).unwrap();
    image.save(&path, ImageFormat::Png).unwrap();
    let before = std::fs::read(&path).unwrap();
    assert!(before.starts_with(&[137, 80, 78, 71]));
    assert_eq!(
        image.save(&path, ImageFormat::Png).unwrap_err().kind(),
        std::io::ErrorKind::AlreadyExists
    );
    assert_eq!(std::fs::read(&path).unwrap(), before);
    std::fs::remove_file(path).unwrap();
}
