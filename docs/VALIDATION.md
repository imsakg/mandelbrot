# Local validation record

This record describes actual local execution, not a historical release or remote CI result. See the reconstructed-date disclosure in [the implementation plan](IMPLEMENTATION.md#history-provenance).

## Passed

- Stable Rust 1.99: formatting, strict all-target/all-feature Clippy, all-target tests, a library doctest and optimized release build.
- Rust 1.88: the complete all-target test suite; this is the declared compiler floor, not a promise about older compilers.
- 76 Rust unit/integration cases, one executable library documentation example and four Python tests. Repeating a suite under another compiler is not counted as extra coverage.
- Naga parsing and validation of `shaders/fractal.wgsl`, plus uniform packing and f32 pixel-spacing regressions. No GPU device was created.
- Five presets rendered to real 128×171 RGB PNGs with a 2×2 sampling grid. Independent Python zlib/CRC decoding checked shape and pixels; one-worker and three-worker images had identical pixel hashes for every preset.
- PNG and PPM encodings of the same Julia scene decoded to identical RGB bytes. Additional PNG tests cross the 65,535-byte stored-DEFLATE block boundary.
- Real POSIX pseudoterminal runs exercised zoom, palette switching, undo, redo, iteration changes and reset. `q`, a standalone Escape byte and Ctrl-C each exited successfully; cursor, alternate screen and persistent termios settings were restored. Darwin's transient PENDIN retype flag is excluded from the equality check.
- CLI checks cover non-TTY rejection, dimension/iteration errors, unknown options, closed pipes, no-clobber output, bounded recipe files, replay overrides and strict hexadecimal RGB parsing.
- Atomic export fault injection preserved a concurrent destination sentinel and removed only the owned temporary.

## Commands

```sh
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo test --locked --doc
cargo +1.88.0 test --locked --all-targets --all-features
cargo build --locked --release
# For a custom CARGO_TARGET_DIR, point MANDELBROT_BIN at its release binary.
python3 -m unittest discover -s tests -p 'test_*.py'
```

## Not claimed

Remote CI has not been run as part of the local reconstruction. Linux/macOS CI jobs are configured but only local macOS execution is evidenced here. There is no hardware GPU execution, arbitrary-precision zoom, optimized PNG compression or general-purpose PNG decoder. Finite iteration caps are not mathematical set-membership proofs. Process termination by an unhandled external signal cannot guarantee destructor cleanup.
