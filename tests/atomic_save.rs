use mandelbrot::{Image, ImageFormat, Rgb};
#[test]
fn atomic_export_cleans_owned_siblings_after_success_and_publication_failure() {
    let dir = std::env::temp_dir().join(format!("mandelbrot-atomic-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("scene.ppm");
    let image = Image::new(1, 1, vec![Rgb(1, 2, 3)]).unwrap();
    image.save(&path, ImageFormat::Ppm).unwrap();
    assert!(image.save(&path, ImageFormat::Ppm).is_err());
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
    std::fs::remove_dir_all(dir).unwrap();
}
