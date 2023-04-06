use std::process::{Command, Stdio};
#[test]
fn process_errors_are_actionable_and_binary_exports_do_not_pollute_stdout() {
    let bin = env!("CARGO_BIN_EXE_mandelbrot");
    for args in [
        ["--width", "0"],
        ["--julia", "nan,0"],
        ["--output", "bad.jpg"],
        ["--missing", "x"],
    ] {
        let out = Command::new(bin).args(args).output().unwrap();
        assert_eq!(out.status.code(), Some(2));
        assert!(out.stdout.is_empty());
        assert!(!out.stderr.is_empty());
    }
    let path = std::env::temp_dir().join(format!("mandelbrot-cli-{}.ppm", std::process::id()));
    let out = Command::new(bin)
        .args([
            "--width", "12", "--height", "6", "--preset", "julia", "--output",
        ])
        .arg(&path)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(out.stdout.is_empty());
    assert!(std::fs::read(&path)
        .unwrap()
        .starts_with(b"P6\n12 6\n255\n"));
    let old = std::fs::read(&path).unwrap();
    let out = Command::new(bin)
        .arg("--output")
        .arg(&path)
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert_eq!(std::fs::read(&path).unwrap(), old);
    std::fs::remove_file(path).unwrap();
}
#[test]
fn closed_output_pipe_is_a_clean_exit_not_a_panic() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mandelbrot"))
        .args(["--width", "64", "--height", "32"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}
