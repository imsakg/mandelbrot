use mandelbrot::cli::Options;
use std::process::Command;
#[test]
fn interactive_mode_is_explicit_and_rejects_nonterminal_pipes() {
    assert!(Options::parse(["--interactive"]).unwrap().interactive);
    assert!(Options::parse(["--interactive", "--output", "scene.png"]).is_err());
    let result = Command::new(env!("CARGO_BIN_EXE_mandelbrot"))
        .arg("--interactive")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8(result.stderr)
        .unwrap()
        .contains("terminal"));
}
