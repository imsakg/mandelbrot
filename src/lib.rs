//! Pure escape-time fractal kernels, independent of terminal I/O.

use num::complex::Complex64;
mod navigation;
mod palette;
mod render;
pub use palette::{ColorSettings, Palette, Rgb};
mod settings;
pub use navigation::Navigation;
pub use render::{render, render_rgb, Frame, RenderSettings, Statistics, MAX_PIXELS};
mod viewport;
pub use settings::{Error, Fractal, IterationSettings};
pub use viewport::Viewport;

pub fn iterate(re: f64, im: f64, settings: &IterationSettings) -> Result<Escape, Error> {
    settings.validate()?;
    if !re.is_finite() || !im.is_finite() {
        return Err(Error("coordinates must be finite".into()));
    }
    Ok(sample(re, im, settings))
}

pub(crate) fn sample(re: f64, im: f64, settings: &IterationSettings) -> Escape {
    orbit(
        re,
        im,
        settings.max_iterations,
        settings.escape_radius,
        settings.fractal,
    )
}

/// A capped orbit. A bounded result is not proof of set membership.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Escape {
    pub iterations: u32,
    pub escaped: bool,
    pub norm_sqr: f64,
    pub distance: Option<f64>,
}

impl Escape {
    /// Continuous escape count. Overflow and invalid powers fall back to integer counts.
    pub fn smooth(self, power: u32) -> f64 {
        if !self.escaped || power < 2 {
            return self.iterations as f64;
        }
        let value =
            self.iterations as f64 + 1.0 - (0.5 * self.norm_sqr.ln()).ln() / (power as f64).ln();
        if value.is_finite() {
            value
        } else {
            self.iterations as f64
        }
    }
}

/// Analytic Mandelbrot interior tests; never apply these to Julia or Burning Ship.
pub fn in_main_bulbs(re: f64, im: f64) -> bool {
    if !re.is_finite() || !im.is_finite() || re.abs() > 2.0 || im.abs() > 2.0 {
        return false;
    }
    let y2 = im * im;
    let q = (re - 0.25) * (re - 0.25) + y2;
    q * (q + re - 0.25) < 0.25 * y2 || (re + 1.0) * (re + 1.0) + y2 < 0.0625
}

/// Iterate `z = z² + c` from zero, counting completed updates.
pub fn escape_point(re: f64, im: f64, limit: u32) -> Escape {
    orbit(re, im, limit, 2.0, Fractal::Mandelbrot)
}

fn orbit(re: f64, im: f64, limit: u32, radius: f64, fractal: Fractal) -> Escape {
    if fractal == Fractal::Mandelbrot && in_main_bulbs(re, im) {
        return Escape {
            iterations: limit,
            escaped: false,
            norm_sqr: 0.0,
            distance: None,
        };
    }
    let (mut z, c) = match fractal {
        Fractal::Mandelbrot | Fractal::BurningShip | Fractal::Multibrot { .. } => {
            (Complex64::new(0.0, 0.0), Complex64::new(re, im))
        }
        Fractal::Julia { re: cr, im: ci } => (Complex64::new(re, im), Complex64::new(cr, ci)),
    };
    let mut derivative = if matches!(fractal, Fractal::Julia { .. }) {
        Complex64::new(1.0, 0.0)
    } else {
        Complex64::new(0.0, 0.0)
    };
    let power = match fractal {
        Fractal::Multibrot { power } => power,
        _ => 2,
    };
    for iterations in 1..=limit {
        let addition = if matches!(fractal, Fractal::Julia { .. }) {
            0.0
        } else {
            1.0
        };
        derivative = (power as f64) * z.powu(power - 1) * derivative + addition;
        if fractal == Fractal::BurningShip {
            z = Complex64::new(z.re.abs(), z.im.abs());
        }
        z = match fractal {
            Fractal::Multibrot { power } if power != 2 => z.powu(power) + c,
            _ => z * z + c,
        };
        if z.norm_sqr() > radius * radius {
            return Escape {
                iterations,
                escaped: true,
                norm_sqr: z.norm_sqr(),
                distance: {
                    let r = z.norm();
                    let d = r * r.ln() / derivative.norm();
                    if fractal != Fractal::BurningShip && d.is_finite() && d > 0.0 {
                        Some(d)
                    } else {
                        None
                    }
                },
            };
        }
    }
    Escape {
        iterations: limit,
        escaped: false,
        norm_sqr: z.norm_sqr(),
        distance: None,
    }
}
