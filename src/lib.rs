//! Pure escape-time fractal kernels, independent of terminal I/O.

use num::complex::Complex64;
mod settings;
pub use settings::{Error, Fractal, IterationSettings};

pub fn iterate(re: f64, im: f64, settings: &IterationSettings) -> Result<Escape, Error> {
    settings.validate()?;
    if !re.is_finite() || !im.is_finite() {
        return Err(Error("coordinates must be finite".into()));
    }
    Ok(orbit(
        re,
        im,
        settings.max_iterations,
        settings.escape_radius,
        settings.fractal,
    ))
}

/// A capped orbit. A bounded result is not proof of set membership.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Escape {
    pub iterations: u32,
    pub escaped: bool,
}

/// Iterate `z = z² + c` from zero, counting completed updates.
pub fn escape_point(re: f64, im: f64, limit: u32) -> Escape {
    orbit(re, im, limit, 2.0, Fractal::Mandelbrot)
}

fn orbit(re: f64, im: f64, limit: u32, radius: f64, fractal: Fractal) -> Escape {
    let (mut z, c) = match fractal {
        Fractal::Mandelbrot | Fractal::BurningShip => {
            (Complex64::new(0.0, 0.0), Complex64::new(re, im))
        }
        Fractal::Julia { re: cr, im: ci } => (Complex64::new(re, im), Complex64::new(cr, ci)),
    };
    for iterations in 1..=limit {
        if fractal == Fractal::BurningShip {
            z = Complex64::new(z.re.abs(), z.im.abs());
        }
        z = z * z + c;
        if z.norm_sqr() > radius * radius {
            return Escape {
                iterations,
                escaped: true,
            };
        }
    }
    Escape {
        iterations: limit,
        escaped: false,
    }
}
