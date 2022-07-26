use mandelbrot::{iterate, render, RenderSettings};

#[test]
fn flat_render_matches_direct_orbits_in_row_major_order() {
    let s = RenderSettings {
        width: 9,
        height: 7,
        ..Default::default()
    };
    let frame = render(&s).unwrap();
    assert_eq!(
        (frame.width(), frame.height(), frame.samples().len()),
        (9, 7, 63)
    );
    for y in 0..s.height {
        for x in 0..s.width {
            let (re, im) = s.viewport.pixel_center(x, y, s.width, s.height).unwrap();
            assert_eq!(
                frame.samples()[y * s.width + x],
                iterate(re, im, &s.iterations).unwrap()
            );
        }
    }
}
