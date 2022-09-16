use crate::{Error, Fractal, Frame};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSettings {
    pub palette: Palette,
    pub smooth: bool,
    pub cycles: f64,
    pub shift: f64,
    pub gamma: f64,
    pub histogram: bool,
}
impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            palette: Palette::Ocean,
            smooth: true,
            cycles: 1.0,
            shift: 0.0,
            gamma: 1.0,
            histogram: false,
        }
    }
}
impl ColorSettings {
    pub fn validate(&self) -> Result<(), Error> {
        if !self.cycles.is_finite()
            || self.cycles <= 0.0
            || self.cycles > 10000.0
            || !self.shift.is_finite()
        {
            return Err(Error(
                "palette cycles must be in (0,10000] and shift must be finite".into(),
            ));
        }
        if !self.gamma.is_finite() || self.gamma <= 0.0 || self.gamma > 100.0 {
            return Err(Error("gamma must be in (0,100]".into()));
        }
        Ok(())
    }
}
impl Frame {
    pub fn colorize(&self, colors: &ColorSettings) -> Result<Vec<Rgb>, Error> {
        colors.validate()?;
        let limit = self.settings().iterations.max_iterations as f64;
        let power = match self.settings().iterations.fractal {
            Fractal::Multibrot { power } => power,
            _ => 2,
        };
        let mut equalized = std::collections::BTreeMap::new();
        if colors.histogram {
            let mut bins = std::collections::BTreeMap::<u32, usize>::new();
            for p in self.samples().iter().filter(|p| p.escaped) {
                *bins.entry(p.iterations).or_default() += 1;
            }
            let total = bins.values().sum::<usize>() as f64;
            let mut cumulative = 0.0;
            for (iteration, count) in bins {
                cumulative += count as f64;
                equalized.insert(iteration, (cumulative - count as f64 / 2.0) / total);
            }
        }
        Ok(self
            .samples()
            .iter()
            .map(|p| {
                if !p.escaped {
                    Rgb(0, 0, 0)
                } else {
                    let value = if colors.smooth {
                        p.smooth(power)
                    } else {
                        p.iterations as f64
                    };
                    colors.palette.sample(
                        (equalized
                            .get(&p.iterations)
                            .copied()
                            .unwrap_or(value / limit)
                            * colors.cycles
                            + colors.shift.rem_euclid(1.0))
                        .rem_euclid(1.0),
                    )
                }
            })
            .map(|rgb| {
                let adjust =
                    |v: u8| ((v as f64 / 255.0).powf(1.0 / colors.gamma) * 255.0).round() as u8;
                Rgb(adjust(rgb.0), adjust(rgb.1), adjust(rgb.2))
            })
            .collect())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Palette {
    Ocean,
    Fire,
    Gray,
    Viridis,
}
impl Palette {
    pub const ALL: [Self; 4] = [Self::Ocean, Self::Fire, Self::Gray, Self::Viridis];
    pub fn name(self) -> &'static str {
        match self {
            Self::Ocean => "ocean",
            Self::Fire => "fire",
            Self::Gray => "gray",
            Self::Viridis => "viridis",
        }
    }
    pub fn parse(name: &str) -> Result<Self, Error> {
        Self::ALL
            .into_iter()
            .find(|p| p.name() == name)
            .ok_or_else(|| Error(format!("unknown palette: {name}")))
    }
    pub fn sample(self, t: f64) -> Rgb {
        let stops: &[Rgb] = match self {
            Self::Ocean => &[
                Rgb(0, 7, 35),
                Rgb(0, 80, 150),
                Rgb(40, 200, 210),
                Rgb(245, 250, 220),
            ],
            Self::Fire => &[
                Rgb(15, 0, 20),
                Rgb(160, 10, 45),
                Rgb(250, 100, 0),
                Rgb(255, 245, 170),
            ],
            Self::Gray => &[Rgb(0, 0, 0), Rgb(255, 255, 255)],
            Self::Viridis => &[
                Rgb(68, 1, 84),
                Rgb(59, 82, 139),
                Rgb(33, 145, 140),
                Rgb(94, 201, 98),
                Rgb(253, 231, 37),
            ],
        };
        let t = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) };
        let position = t * (stops.len() - 1) as f64;
        let i = (position as usize).min(stops.len() - 2);
        stops[i].lerp(stops[i + 1], position - i as f64)
    }
}

/// An sRGB triplet. Palette interpolation is display-space, not linear-light.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);
impl Rgb {
    pub fn bytes(self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
    pub fn lerp(self, other: Self, t: f64) -> Self {
        let t = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) };
        let mix = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * t).round() as u8;
        Self(
            mix(self.0, other.0),
            mix(self.1, other.1),
            mix(self.2, other.2),
        )
    }
}
