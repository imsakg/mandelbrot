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
pub struct IterationSettings {
    pub max_iterations: u32,
}
impl Default for IterationSettings {
    fn default() -> Self {
        Self {
            max_iterations: 256,
        }
    }
}
impl IterationSettings {
    pub fn validate(&self) -> Result<(), Error> {
        if self.max_iterations == 0 || self.max_iterations > 10_000_000 {
            return Err(Error("iterations must be in 1..=10000000".into()));
        }
        Ok(())
    }
}
