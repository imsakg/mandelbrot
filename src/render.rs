use crate::{Error, IterationSettings, Viewport};

pub const MAX_PIXELS: usize = 16_000_000;
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
