use std::process::Command;
#[test]
fn optional_timing_stats_go_to_stderr_without_altering_the_frame() {
    let bin = env!("CARGO_BIN_EXE_mandelbrot");
    let args = ["--width", "12", "--height", "6"];
    let plain = Command::new(bin).args(args).output().unwrap();
    let stats = Command::new(bin)
        .args(args)
        .arg("--stats")
        .output()
        .unwrap();
    assert!(stats.status.success());
    assert_eq!(plain.stdout, stats.stdout);
    assert!(String::from_utf8(stats.stderr)
        .unwrap()
        .contains("elapsed_seconds="));
}
