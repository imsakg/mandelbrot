use crate::{sample, Error, Escape, IterationSettings, Viewport};

pub const MAX_PIXELS: usize = 16_000_000;

/// Flat, row-major orbit data; dimensions cannot drift from buffer length.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    settings: RenderSettings,
    samples: Vec<Escape>,
}
impl Frame {
    pub fn width(&self) -> usize {
        self.settings.width
    }
    pub fn height(&self) -> usize {
        self.settings.height
    }
    pub fn samples(&self) -> &[Escape] {
        &self.samples
    }
    pub fn settings(&self) -> &RenderSettings {
        &self.settings
    }
}

pub fn render(settings: &RenderSettings) -> Result<Frame, Error> {
    let area = settings.validate()?;
    let mut samples = Vec::new();
    samples
        .try_reserve_exact(area)
        .map_err(|e| Error(format!("cannot allocate frame: {e}")))?;
    for y in 0..settings.height {
        for x in 0..settings.width {
            let (re, im) = settings.viewport.coordinate(
                (x as f64 + 0.5) / settings.width as f64,
                (y as f64 + 0.5) / settings.height as f64,
            );
            samples.push(sample(re, im, &settings.iterations));
        }
    }
    Ok(Frame {
        settings: settings.clone(),
        samples,
    })
}
#[derive(Clone, Debug, PartialEq)]
pub struct RenderSettings {
    pub width: usize,
    pub height: usize,
    pub viewport: Viewport,
    pub iterations: IterationSettings,
}
impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            viewport: Viewport::default(),
            iterations: IterationSettings::default(),
        }
    }
}
impl RenderSettings {
    pub fn validate(&self) -> Result<usize, Error> {
        self.iterations.validate()?;
        let area = self
            .width
            .checked_mul(self.height)
            .filter(|n| *n > 0 && *n <= MAX_PIXELS)
            .ok_or_else(|| Error(format!("image area must be in 1..={MAX_PIXELS}")))?;
        self.viewport.validate_resolution(self.width, self.height)?;
        Ok(area)
    }
}
