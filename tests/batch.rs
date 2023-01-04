use std::process::Command;
#[test]
fn batch_ascii_finishes_without_keyboard_input_and_help_is_discoverable() {
    let bin = env!("CARGO_BIN_EXE_mandelbrot");
    let out = Command::new(bin)
        .args(["--width", "12", "--height", "6"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    assert_eq!(text.lines().count(), 6);
    assert!(text.lines().all(|line| line.len() == 12));
    let help = Command::new(bin).arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8(help.stdout).unwrap().contains("--output"));
}
