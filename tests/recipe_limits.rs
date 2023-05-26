use std::process::Command;
#[test]
fn recipe_inputs_have_a_byte_limit_and_cannot_chain_file_reads() {
    let path = std::env::temp_dir().join(format!(
        "mandelbrot-recipe-limits-{}.txt",
        std::process::id()
    ));
    for bytes in [
        vec![b' '; 65_537],
        b"--recipe private.txt".to_vec(),
        vec![255],
    ] {
        std::fs::write(&path, bytes).unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_mandelbrot"))
            .arg("--recipe")
            .arg(&path)
            .output()
            .unwrap();
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
        assert!(!out.stderr.is_empty());
    }
    std::fs::remove_file(path).unwrap();
}
