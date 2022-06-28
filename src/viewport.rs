use crate::Error;

/// Ordered rectangular bounds in the complex plane; image rows increase imaginary values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}
impl Default for Viewport {
    fn default() -> Self {
        Self {
            x_min: -2.0,
            x_max: 1.0,
            y_min: -1.0,
            y_max: 1.0,
        }
    }
}
impl Viewport {
    pub fn zoom_at(self, factor: f64, u: f64, v: f64) -> Result<Self, Error> {
        if !factor.is_finite()
            || factor <= 0.0
            || !(0.0..=1.0).contains(&u)
            || !(0.0..=1.0).contains(&v)
        {
            return Err(Error(
                "zoom requires a positive factor and anchors in 0..=1".into(),
            ));
        }
        let (x, y) = self.coordinate(u, v);
        let (dx, dy) = self.spans();
        let w = dx / factor;
        let h = dy / factor;
        Self::new(x - u * w, x + (1.0 - u) * w, y - v * h, y + (1.0 - v) * h)
    }
    /// Factors greater than one zoom in; factors between zero and one zoom out.
    pub fn zoom(self, factor: f64) -> Result<Self, Error> {
        if !factor.is_finite() || factor <= 0.0 {
            return Err(Error("zoom factor must be finite and positive".into()));
        }
        let (cx, cy) = self.coordinate(0.5, 0.5);
        let (dx, dy) = self.spans();
        Self::new(
            cx - dx / factor / 2.0,
            cx + dx / factor / 2.0,
            cy - dy / factor / 2.0,
            cy + dy / factor / 2.0,
        )
    }
    pub fn pixel_center(
        self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Result<(f64, f64), Error> {
        if width == 0 || height == 0 || x >= width || y >= height {
            return Err(Error("pixel is outside nonempty image dimensions".into()));
        }
        Ok(self.coordinate(
            (x as f64 + 0.5) / width as f64,
            (y as f64 + 0.5) / height as f64,
        ))
    }
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Result<Self, Error> {
        let values = [x_min, x_max, y_min, y_max];
        let dx = x_max - x_min;
        let dy = y_max - y_min;
        if values.iter().any(|v| !v.is_finite())
            || dx <= 0.0
            || dy <= 0.0
            || !dx.is_finite()
            || !dy.is_finite()
        {
            return Err(Error(
                "bounds must be finite and strictly increasing".into(),
            ));
        }
        Ok(Self {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
    pub fn bounds(self) -> [f64; 4] {
        [self.x_min, self.x_max, self.y_min, self.y_max]
    }
    pub fn spans(self) -> (f64, f64) {
        (self.x_max - self.x_min, self.y_max - self.y_min)
    }
    /// Map normalized image coordinates. Pixel-center callers use `(x + 0.5)/width`.
    pub fn coordinate(self, u: f64, v: f64) -> (f64, f64) {
        let (dx, dy) = self.spans();
        (self.x_min + dx * u, self.y_min + dy * v)
    }
}
