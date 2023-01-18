//! Pure keyboard transitions. Invalid moves leave the prior settings untouched.
use crate::{Error, RenderSettings, Viewport};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    None,
    Redraw,
    Quit,
}
pub fn apply(settings: &mut RenderSettings, key: char) -> Result<Action, Error> {
    let mut next = settings.clone();
    match key {
        'q' | '\u{1b}' => return Ok(Action::Quit),
        'w' => next.viewport = next.viewport.pan(0.0, -0.1)?,
        's' => next.viewport = next.viewport.pan(0.0, 0.1)?,
        'a' => next.viewport = next.viewport.pan(-0.1, 0.0)?,
        'd' => next.viewport = next.viewport.pan(0.1, 0.0)?,
        'k' | '+' => next.viewport = next.viewport.zoom(2.0)?,
        'j' | '-' => next.viewport = next.viewport.zoom(0.5)?,
        'r' => next.viewport = Viewport::default(),
        ']' => {
            next.iterations.max_iterations = next
                .iterations
                .max_iterations
                .saturating_mul(2)
                .min(10_000_000)
        }
        '[' => next.iterations.max_iterations = (next.iterations.max_iterations / 2).max(1),
        _ => return Ok(Action::None),
    }
    next.validate()?;
    *settings = next;
    Ok(Action::Redraw)
}
