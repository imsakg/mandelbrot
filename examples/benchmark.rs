//! `cargo run --release --example benchmark -- --width 320 --height 240 --threads 4`
//! Simple wall-time sampler, not a statistical benchmarking framework.
use mandelbrot::{cli::Options, render_parallel};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse(std::env::args().skip(1))?;
    std::hint::black_box(render_parallel(&options.render, options.threads)?);
    for trial in 1..=3 {
        let start = std::time::Instant::now();
        let frame = render_parallel(&options.render, options.threads)?;
        let elapsed = start.elapsed();
        let stats = frame.statistics();
        std::hint::black_box(&frame);
        println!("trial={trial} size={}x{} workers={} seconds={:.6} escaped={} bounded={} iteration_sum={}",frame.width(),frame.height(),options.threads,elapsed.as_secs_f64(),stats.escaped,stats.bounded,stats.iteration_sum);
    }
    Ok(())
}
