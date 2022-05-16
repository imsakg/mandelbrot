use mandelbrot::{iterate, Fractal, IterationSettings};

#[test]
fn burning_ship_matches_an_independent_real_recurrence() {
    let s = IterationSettings {
        fractal: Fractal::BurningShip,
        max_iterations: 80,
        ..Default::default()
    };
    for (cx, cy) in [(-1.8, -0.03), (-1.7, 0.1), (0.0, 0.0), (0.4, 0.6)] {
        let (mut x, mut y) = (0.0_f64, 0.0_f64);
        let mut expected = (80, false);
        for i in 1..=80 {
            let nx = x * x - y * y + cx;
            y = 2.0 * (x * y).abs() + cy;
            x = nx;
            if x * x + y * y > 4.0 {
                expected = (i, true);
                break;
            }
        }
        let got = iterate(cx, cy, &s).unwrap();
        assert_eq!((got.iterations, got.escaped), expected);
    }
}
