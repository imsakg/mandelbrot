//! Pure escape-time fractal kernels, independent of terminal I/O.

use num::complex::Complex64;

/// A capped orbit. A bounded result is not proof of set membership.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Escape {
    pub iterations: u32,
    pub escaped: bool,
}

/// Iterate `z = z² + c` from zero, counting completed updates.
pub fn escape_point(re: f64, im: f64, limit: u32) -> Escape {
    let c = Complex64::new(re, im);
    let mut z = Complex64::new(0.0, 0.0);
    for iterations in 1..=limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
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
