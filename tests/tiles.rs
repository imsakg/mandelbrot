use mandelbrot::tiles;
#[test]
fn edge_tiles_cover_every_pixel_exactly_once() {
    let mut seen = vec![0; 7 * 5];
    for tile in tiles(7, 5, 3, 2).unwrap() {
        for y in tile.y..tile.y + tile.height {
            for x in tile.x..tile.x + tile.width {
                seen[y * 7 + x] += 1;
            }
        }
    }
    assert!(seen.iter().all(|v| *v == 1));
    assert!(tiles(7, 5, 0, 2).is_err());
    assert!(tiles(0, 5, 2, 2).is_err());
}
