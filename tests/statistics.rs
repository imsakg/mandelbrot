use mandelbrot::{render, RenderSettings};
#[test]
fn frame_statistics_account_for_every_sample_and_iteration_budget() {
    let f = render(&RenderSettings {
        width: 9,
        height: 7,
        ..Default::default()
    })
    .unwrap();
    let stats = f.statistics();
    assert_eq!(stats.escaped + stats.bounded, 63);
    assert!(stats.escaped > 0 && stats.bounded > 0);
    assert_eq!(
        stats.iteration_sum,
        f.samples().iter().map(|p| p.iterations as u64).sum::<u64>()
    );
    assert_eq!(f.histogram().values().sum::<usize>(), 63);
}
