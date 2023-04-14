use mandelbrot::{escape_point, Viewport};
#[test]
fn lattice_orbits_match_real_arithmetic_and_conjugation() {
    for iy in -12..=12 {
        for ix in -18..=6 {
            let cx = ix as f64 / 10.0;
            let cy = iy as f64 / 10.0;
            let (mut x, mut y) = (0.0, 0.0);
            let mut expected = (100, false);
            for n in 1..=100 {
                let nx = x * x - y * y + cx;
                y = 2.0 * x * y + cy;
                x = nx;
                if x * x + y * y > 4.0 {
                    expected = (n, true);
                    break;
                }
            }
            let p = escape_point(cx, cy, 100);
            assert_eq!((p.iterations, p.escaped), expected, "{cx},{cy}");
            assert_eq!(p, escape_point(cx, -cy, 100));
            if escape_point(cx, cy, 20).escaped {
                assert!(p.escaped);
            }
        }
    }
}
#[test]
fn inverse_navigation_preserves_lattice_centers_within_roundoff() {
    let v = Viewport::default();
    for i in 1..20 {
        let f = i as f64 / 7.0;
        let p = v
            .zoom(f)
            .unwrap()
            .pan(0.2, -0.1)
            .unwrap()
            .pan(-0.2, 0.1)
            .unwrap()
            .zoom(1.0 / f)
            .unwrap();
        for (a, b) in v.bounds().iter().zip(p.bounds()) {
            assert!((a - b).abs() < 1e-12);
        }
    }
}
