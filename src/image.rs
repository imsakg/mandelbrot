use crate::{Error, Rgb, MAX_PIXELS};

/// A checked RGB image. Encoders can rely on exact row-major pixel length.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Rgb>,
}
impl Image {
    pub fn new(width: usize, height: usize, pixels: Vec<Rgb>) -> Result<Self, Error> {
        let area = width
            .checked_mul(height)
            .filter(|n| *n > 0 && *n <= MAX_PIXELS)
            .ok_or_else(|| Error("invalid image dimensions".into()))?;
        if area != pixels.len() {
            return Err(Error(
                "RGB buffer length does not match image dimensions".into(),
            ));
        }
        Ok(Self {
            width,
            height,
            pixels,
        })
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn pixels(&self) -> &[Rgb] {
        &self.pixels
    }
}
