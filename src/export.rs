use crate::{Error, Image};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    Ppm,
    Png,
}
impl ImageFormat {
    pub fn from_path(path: &std::path::Path) -> Result<Self, Error> {
        match path
            .extension()
            .and_then(|v| v.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("ppm") => Ok(Self::Ppm),
            Some("png") => Ok(Self::Png),
            _ => Err(Error("output extension must be .png or .ppm".into())),
        }
    }
}

use std::io::{self, Write};

fn write_chunk<W: Write>(writer: &mut W, name: &[u8; 4], data: &[u8]) -> io::Result<()> {
    let length = u32::try_from(data.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "PNG chunk is too long"))?;
    writer.write_all(&length.to_be_bytes())?;
    writer.write_all(name)?;
    writer.write_all(data)?;
    let mut checked = Vec::with_capacity(4 + data.len());
    checked.extend_from_slice(name);
    checked.extend_from_slice(data);
    writer.write_all(&crate::checksum::crc32(&checked).to_be_bytes())
}

impl Image {
    /// Save without overwriting existing paths; remove partial output on encode/flush errors.
    pub fn save(&self, path: &std::path::Path, format: ImageFormat) -> io::Result<()> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        let mut writer = io::BufWriter::new(file);
        let result = self
            .write(&mut writer, format)
            .and_then(|()| writer.flush());
        drop(writer);
        if result.is_err() {
            let _ = std::fs::remove_file(path);
        }
        result
    }
    pub fn write<W: Write>(&self, writer: &mut W, format: ImageFormat) -> io::Result<()> {
        match format {
            ImageFormat::Ppm => self.write_ppm(writer),
            ImageFormat::Png => self.write_png(writer),
        }
    }
    /// Write RGB8 PNG with uncompressed DEFLATE blocks. Portable, not size-optimized.
    pub fn write_png<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"\x89PNG\r\n\x1a\n")?;
        let mut header = Vec::with_capacity(13);
        header.extend_from_slice(&(self.width() as u32).to_be_bytes());
        header.extend_from_slice(&(self.height() as u32).to_be_bytes());
        header.extend_from_slice(&[8, 2, 0, 0, 0]);
        write_chunk(writer, b"IHDR", &header)?;
        let mut raw = Vec::with_capacity(self.pixels().len() * 3 + self.height());
        for row in self.pixels().chunks(self.width()) {
            raw.push(0);
            for pixel in row {
                raw.extend_from_slice(&pixel.bytes());
            }
        }
        let mut zlib = vec![0x78, 0x01];
        let blocks = raw.chunks(65535);
        let count = blocks.len();
        for (i, block) in blocks.enumerate() {
            let len = block.len() as u16;
            zlib.push(u8::from(i + 1 == count));
            zlib.extend_from_slice(&len.to_le_bytes());
            zlib.extend_from_slice(&(!len).to_le_bytes());
            zlib.extend_from_slice(block);
        }
        zlib.extend_from_slice(&crate::checksum::adler32(&raw).to_be_bytes());
        write_chunk(writer, b"IDAT", &zlib)?;
        write_chunk(writer, b"IEND", &[])
    }

    /// Write a portable P6 RGB pixmap, propagating all writer errors.
    pub fn write_ppm<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        write!(writer, "P6\n{} {}\n255\n", self.width(), self.height())?;
        for pixel in self.pixels() {
            writer.write_all(&pixel.bytes())?;
        }
        Ok(())
    }
}
