use mandelbrot::{render, render_parallel, Fractal, RenderSettings};
#[test]
fn parallel_frames_are_identical_for_uneven_rows_and_every_fractal() {
    for fractal in [
        Fractal::Mandelbrot,
        Fractal::Julia {
            re: -0.8,
            im: 0.156,
        },
        Fractal::BurningShip,
        Fractal::Multibrot { power: 3 },
    ] {
        let mut s = RenderSettings {
            width: 13,
            height: 7,
            ..Default::default()
        };
        s.iterations.fractal = fractal;
        let serial = render(&s).unwrap();
        for workers in [1, 2, 8] {
            assert_eq!(serial, render_parallel(&s, workers).unwrap());
        }
    }
    assert!(render_parallel(&RenderSettings::default(), 0).is_err());
    assert!(render_parallel(&RenderSettings::default(), 65).is_err());
}
