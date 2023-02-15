//! Small curated starting points, not guarantees of arbitrary deep-zoom accuracy.
use crate::{Error, Fractal, RenderSettings, Viewport};
#[derive(Clone, Copy, Debug)]
pub struct Preset {
    pub name: &'static str,
    pub bounds: [f64; 4],
    pub fractal: Fractal,
    pub iterations: u32,
}
pub const ALL: [Preset; 5] = [
    Preset {
        name: "overview",
        bounds: [-2.0, 1.0, -1.0, 1.0],
        fractal: Fractal::Mandelbrot,
        iterations: 256,
    },
    Preset {
        name: "seahorse",
        bounds: [-0.80, -0.70, 0.05, 0.15],
        fractal: Fractal::Mandelbrot,
        iterations: 1200,
    },
    Preset {
        name: "elephant",
        bounds: [0.25, 0.30, -0.02, 0.02],
        fractal: Fractal::Mandelbrot,
        iterations: 800,
    },
    Preset {
        name: "julia",
        bounds: [-1.6, 1.6, -1.2, 1.2],
        fractal: Fractal::Julia {
            re: -0.8,
            im: 0.156,
        },
        iterations: 512,
    },
    Preset {
        name: "ship",
        bounds: [-1.85, -1.70, -0.1, 0.025],
        fractal: Fractal::BurningShip,
        iterations: 600,
    },
];
pub fn apply(name: &str, settings: &mut RenderSettings) -> Result<(), Error> {
    let preset = ALL
        .iter()
        .find(|p| p.name == name)
        .ok_or_else(|| Error(format!("unknown preset: {name}")))?;
    let [a, b, c, d] = preset.bounds;
    let mut next = settings.clone();
    next.viewport = Viewport::new(a, b, c, d)?;
    next.iterations.fractal = preset.fractal;
    next.iterations.max_iterations = preset.iterations;
    next.validate()?;
    *settings = next;
    Ok(())
}
