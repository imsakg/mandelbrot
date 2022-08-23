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
