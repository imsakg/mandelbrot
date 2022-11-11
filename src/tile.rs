use crate::{Error, MAX_PIXELS};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

/// Plan clipped row-major tiles for CPU scheduling or a future GPU dispatch.
pub fn tiles(
    width: usize,
    height: usize,
    tile_width: usize,
    tile_height: usize,
) -> Result<Vec<Tile>, Error> {
    if tile_width == 0
        || tile_height == 0
        || width
            .checked_mul(height)
            .filter(|n| *n > 0 && *n <= MAX_PIXELS)
            .is_none()
    {
        return Err(Error(
            "tiles require nonzero dimensions within the image budget".into(),
        ));
    }
    let mut out = Vec::new();
    for y in (0..height).step_by(tile_height) {
        for x in (0..width).step_by(tile_width) {
            out.push(Tile {
                x,
                y,
                width: tile_width.min(width - x),
                height: tile_height.min(height - y),
            });
        }
    }
    Ok(out)
}
