//! `cargo run --release --example render -- image.png`
use mandelbrot::{render_rgb_parallel, ColorSettings, Image, ImageFormat, RenderSettings};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .map(std::path::PathBuf::from)
        .ok_or("usage: render OUTPUT.png")?;
    let settings = RenderSettings {
        width: 320,
        height: 240,
        ..Default::default()
    };
    let pixels = render_rgb_parallel(&settings, &ColorSettings::default(), 2, 2)?;
    Image::new(settings.width, settings.height, pixels)?
        .save(&path, ImageFormat::from_path(&path)?)?;
    println!(
        "saved {} × {} to {}",
        settings.width,
        settings.height,
        path.display()
    );
    Ok(())
}
