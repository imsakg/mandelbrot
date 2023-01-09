use crate::{sample, Error, Escape, IterationSettings, Viewport};

pub const MAX_PIXELS: usize = 16_000_000;

/// Render an n×n regular subpixel grid and average colors, with n in 1..=4.
pub fn render_rgb(
    settings: &RenderSettings,
    colors: &crate::ColorSettings,
    samples: u8,
) -> Result<Vec<crate::Rgb>, Error> {
    render_rgb_parallel(settings, colors, samples, 1)
}
pub fn render_rgb_parallel(
    settings: &RenderSettings,
    colors: &crate::ColorSettings,
    samples: u8,
    workers: usize,
) -> Result<Vec<crate::Rgb>, Error> {
    settings.validate()?;
    colors.validate()?;
    if !(1..=4).contains(&samples) {
        return Err(Error("supersampling must be in 1..=4".into()));
    }
    let n = samples as usize;
    let mut high = settings.clone();
    high.width = high
        .width
        .checked_mul(n)
        .ok_or_else(|| Error("supersampling width overflow".into()))?;
    high.height = high
        .height
        .checked_mul(n)
        .ok_or_else(|| Error("supersampling height overflow".into()))?;
    let pixels = render_parallel(&high, workers)?.colorize(colors)?;
    let mut result = Vec::with_capacity(settings.width * settings.height);
    let count = (n * n) as u32;
    for y in 0..settings.height {
        for x in 0..settings.width {
            let mut sum = [0u32; 3];
            for sy in 0..n {
                for sx in 0..n {
                    for (channel, value) in sum
                        .iter_mut()
                        .zip(pixels[(y * n + sy) * high.width + x * n + sx].bytes())
                    {
                        *channel += value as u32;
                    }
                }
            }
            result.push(crate::Rgb(
                ((sum[0] + count / 2) / count) as u8,
                ((sum[1] + count / 2) / count) as u8,
                ((sum[2] + count / 2) / count) as u8,
            ));
        }
    }
    Ok(result)
}

/// Flat, row-major orbit data; dimensions cannot drift from buffer length.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    settings: RenderSettings,
    samples: Vec<Escape>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Statistics {
    pub escaped: usize,
    pub bounded: usize,
    pub iteration_sum: u64,
}
impl Frame {
    pub fn ascii(&self) -> String {
        const GLYPHS: &[u8] = b" .:-=+*#%@";
        let mut text = String::with_capacity(self.samples.len() + self.height());
        let limit = self.settings.iterations.max_iterations as usize;
        for row in self.samples.chunks(self.width()) {
            for p in row {
                let index = if p.escaped {
                    (p.iterations as usize * (GLYPHS.len() - 2) / limit).min(GLYPHS.len() - 2)
                } else {
                    GLYPHS.len() - 1
                };
                text.push(GLYPHS[index] as char);
            }
            text.push('\n');
        }
        text
    }
    pub fn statistics(&self) -> Statistics {
        let escaped = self.samples.iter().filter(|p| p.escaped).count();
        Statistics {
            escaped,
            bounded: self.samples.len() - escaped,
            iteration_sum: self.samples.iter().map(|p| p.iterations as u64).sum(),
        }
    }
    pub fn histogram(&self) -> std::collections::BTreeMap<u32, usize> {
        let mut bins = std::collections::BTreeMap::new();
        for p in &self.samples {
            *bins.entry(p.iterations).or_default() += 1;
        }
        bins
    }
    pub fn row(&self, y: usize) -> Option<&[Escape]> {
        if y >= self.height() {
            None
        } else {
            Some(&self.samples[y * self.width()..(y + 1) * self.width()])
        }
    }
    pub fn at(&self, x: usize, y: usize) -> Option<&Escape> {
        if x >= self.width() || y >= self.height() {
            None
        } else {
            self.samples.get(y * self.width() + x)
        }
    }
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

pub fn render_parallel(settings: &RenderSettings, workers: usize) -> Result<Frame, Error> {
    let area = settings.validate()?;
    if !(1..=64).contains(&workers) {
        return Err(Error("worker count must be in 1..=64".into()));
    }
    if workers == 1 {
        return render(settings);
    }
    let workers = workers.min(settings.height);
    let rows = settings.height / workers + usize::from(settings.height % workers != 0);
    let mut samples = Vec::new();
    samples
        .try_reserve_exact(area)
        .map_err(|e| Error(format!("cannot allocate frame: {e}")))?;
    samples.resize(
        area,
        Escape {
            iterations: 0,
            escaped: false,
            norm_sqr: 0.0,
            distance: None,
        },
    );
    std::thread::scope(|scope| -> Result<(), Error> {
        let mut handles = Vec::new();
        for (stripe, chunk) in samples.chunks_mut(rows * settings.width).enumerate() {
            let handle = std::thread::Builder::new()
                .name(format!("fractal-{stripe}"))
                .spawn_scoped(scope, move || {
                    for (i, p) in chunk.iter_mut().enumerate() {
                        let x = i % settings.width;
                        let y = stripe * rows + i / settings.width;
                        let (re, im) = settings.viewport.coordinate(
                            (x as f64 + 0.5) / settings.width as f64,
                            (y as f64 + 0.5) / settings.height as f64,
                        );
                        *p = sample(re, im, &settings.iterations);
                    }
                })
                .map_err(|e| Error(format!("cannot spawn render worker: {e}")))?;
            handles.push(handle);
        }
        for handle in handles {
            handle
                .join()
                .map_err(|_| Error("render worker panicked".into()))?;
        }
        Ok(())
    })?;
    Ok(Frame {
        settings: settings.clone(),
        samples,
    })
}

pub fn render(settings: &RenderSettings) -> Result<Frame, Error> {
    render_with_progress(settings, |_, _| true)
}

/// Notify after each completed row; returning false cancels without returning a partial frame.
pub fn render_with_progress<F: FnMut(usize, usize) -> bool>(
    settings: &RenderSettings,
    mut progress: F,
) -> Result<Frame, Error> {
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
        if !progress(y + 1, settings.height) {
            return Err(Error(format!("render canceled after row {}", y + 1)));
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
