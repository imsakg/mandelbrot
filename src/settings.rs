use std::fmt;

/// Invalid numeric configuration or unsupported resource request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error(pub String);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fractal {
    Mandelbrot,
    BurningShip,
    Multibrot { power: u32 },
    Julia { re: f64, im: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IterationSettings {
    pub max_iterations: u32,
    pub escape_radius: f64,
    pub fractal: Fractal,
}
impl Default for IterationSettings {
    fn default() -> Self {
        Self {
            max_iterations: 256,
            escape_radius: 2.0,
            fractal: Fractal::Mandelbrot,
        }
    }
}
impl IterationSettings {
    pub fn validate(&self) -> Result<(), Error> {
        if self.max_iterations == 0 || self.max_iterations > 10_000_000 {
            return Err(Error("iterations must be in 1..=10000000".into()));
        }
        if !self.escape_radius.is_finite()
            || self.escape_radius < 2.0
            || !self.escape_radius.powi(2).is_finite()
        {
            return Err(Error(
                "escape radius must be finite, at least two, and squareable".into(),
            ));
        }
        if let Fractal::Julia { re, im } = self.fractal {
            if !re.is_finite() || !im.is_finite() || !re.hypot(im).powi(2).is_finite() {
                return Err(Error("Julia constant must be finite".into()));
            }
        }
        if let Fractal::Multibrot { power } = self.fractal {
            if !(2..=8).contains(&power) {
                return Err(Error("Multibrot power must be in 2..=8".into()));
            }
        }
        Ok(())
    }
}
