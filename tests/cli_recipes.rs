use std::process::Command;
#[test]
fn printed_recipes_load_as_data_without_shell_execution_and_allow_later_overrides() {
    let bin = env!("CARGO_BIN_EXE_mandelbrot");
    let out = Command::new(bin)
        .args([
            "--preset",
            "julia",
            "--width",
            "9",
            "--height",
            "5",
            "--print-recipe",
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    let path = std::env::temp_dir().join(format!("mandelbrot-view-{}.txt", std::process::id()));
    std::fs::write(&path, &out.stdout).unwrap();
    let loaded = Command::new(bin)
        .arg("--recipe")
        .arg(&path)
        .args(["--width", "11"])
        .output()
        .unwrap();
    assert!(loaded.status.success());
    let text = String::from_utf8(loaded.stdout).unwrap();
    assert_eq!(text.lines().count(), 5);
    assert!(text.lines().all(|line| line.len() == 11));
    std::fs::write(&path, b"--output surprise.png").unwrap();
    let bad = Command::new(bin)
        .arg("--recipe")
        .arg(&path)
        .output()
        .unwrap();
    assert!(!bad.status.success());
    std::fs::remove_file(path).unwrap();
}
