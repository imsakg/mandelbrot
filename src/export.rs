use crate::Image;
use std::io::{self, Write};

impl Image {
    /// Write a portable P6 RGB pixmap, propagating all writer errors.
    pub fn write_ppm<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        write!(writer, "P6\n{} {}\n255\n", self.width(), self.height())?;
        for pixel in self.pixels() {
            writer.write_all(&pixel.bytes())?;
        }
        Ok(())
    }
}
