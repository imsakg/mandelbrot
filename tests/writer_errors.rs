use mandelbrot::{Image, ImageFormat, Rgb};
use std::io::{self, Write};
struct ShortWriter {
    data: Vec<u8>,
    limit: usize,
}
impl Write for ShortWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.data.len() >= self.limit {
            return Err(io::Error::new(io::ErrorKind::WriteZero, "injected limit"));
        }
        let count = bytes.len().min(2).min(self.limit - self.data.len());
        self.data.extend_from_slice(&bytes[..count]);
        Ok(count)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
#[test]
fn encoders_retry_short_writes_and_propagate_midstream_failures() {
    let image = Image::new(4, 3, vec![Rgb(10, 20, 30); 12]).unwrap();
    for format in [ImageFormat::Png, ImageFormat::Ppm] {
        let mut expected = vec![];
        image.write(&mut expected, format).unwrap();
        let mut short = ShortWriter {
            data: vec![],
            limit: usize::MAX,
        };
        image.write(&mut short, format).unwrap();
        assert_eq!(short.data, expected);
        for limit in [0, 5, expected.len() - 1] {
            let mut bad = ShortWriter {
                data: vec![],
                limit,
            };
            assert_eq!(
                image.write(&mut bad, format).unwrap_err().kind(),
                io::ErrorKind::WriteZero
            );
        }
    }
}
