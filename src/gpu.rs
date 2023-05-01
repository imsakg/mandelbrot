//! Host-side hints only: this crate does not create GPU devices or submit GPU work.
use crate::{Error, Fractal, RenderSettings};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpuParams {
    bounds: [f32; 4],
    constant: [f32; 2],
    width: u32,
    height: u32,
    iterations: u32,
    variant: u32,
    power: u32,
    radius_sqr: f32,
}
impl GpuParams {
    pub fn from_settings(s: &RenderSettings) -> Result<Self, Error> {
        s.validate()?;
        let b = s.viewport.bounds();
        let bounds = b.map(|v| v as f32);
        let (dx, dy) = s.viewport.spans();
        let monotone = |min: f32, span: f64, count: usize| {
            let mut last = min;
            for i in 0..count {
                let now = (min as f64 + span * ((i as f64 + 0.5) / count as f64)) as f32;
                if i > 0 && now <= last {
                    return false;
                }
                last = now;
            }
            true
        };
        if bounds.iter().any(|v| !v.is_finite())
            || !monotone(bounds[0], dx, s.width)
            || !monotone(bounds[2], dy, s.height)
        {
            return Err(Error("view exceeds f32 GPU reference precision".into()));
        }
        let (variant, constant, power) = match s.iterations.fractal {
            Fractal::Mandelbrot => (0, [0.0, 0.0], 2),
            Fractal::Julia { re, im } => (1, [re as f32, im as f32], 2),
            Fractal::BurningShip => (2, [0.0, 0.0], 2),
            Fractal::Multibrot { power } => (3, [0.0, 0.0], power),
        };
        let radius = if variant == 1 {
            s.iterations
                .escape_radius
                .max((constant[0] as f64).hypot(constant[1] as f64))
        } else {
            s.iterations.escape_radius
        };
        let radius_sqr = (radius * radius) as f32;
        if constant.iter().any(|v| !v.is_finite()) || !radius_sqr.is_finite() {
            return Err(Error("GPU parameters exceed f32 range".into()));
        }
        Ok(Self {
            bounds,
            constant,
            width: s.width as u32,
            height: s.height as u32,
            iterations: s.iterations.max_iterations,
            variant,
            power,
            radius_sqr,
        })
    }
    pub fn workgroups(self) -> (u32, u32) {
        (self.width.div_ceil(8), self.height.div_ceil(8))
    }
    /// Twelve little-endian words: bounds, Julia constant, dimensions, iteration/variant/power, radius².
    pub fn to_le_bytes(self) -> [u8; 48] {
        let words = [
            self.bounds[0].to_bits(),
            self.bounds[1].to_bits(),
            self.bounds[2].to_bits(),
            self.bounds[3].to_bits(),
            self.constant[0].to_bits(),
            self.constant[1].to_bits(),
            self.width,
            self.height,
            self.iterations,
            self.variant,
            self.power,
            self.radius_sqr.to_bits(),
        ];
        let mut bytes = [0u8; 48];
        for (chunk, word) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(words) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
        bytes
    }
}
