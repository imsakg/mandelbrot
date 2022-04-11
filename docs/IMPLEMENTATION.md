# Mandelbrot implementation plan

> **For Hermes:** Execute the approved scope task-by-task with test-first library changes and small verified commits.

**Goal:** Turn the original terminal sketch into a reusable, tested fractal explorer and image renderer.

**Architecture:** Keep numeric kernels, viewport geometry, palettes, image encoding and CLI parsing separate. The binary is a thin adapter; serial rendering is the correctness oracle for parallel rendering. GPU material is an optional porting reference, not a claim of hardware acceleration.

**Tech stack:** Rust 2021, existing `num` complex arithmetic and `getch` terminal input; standard-library image encoders and scoped workers.

## Sequence and acceptance

1. Extract a pure escape-time library (`src/lib.rs`, `tests/core.rs`). Define iteration semantics, configurable bailout, Julia and Burning Ship recurrences, powers, smooth values and derivative distance estimates. Test bounded, escaping, boundary and invalid input cases before each implementation.
2. Build validated `Viewport` geometry (`src/viewport.rs`, `tests/viewport.rs`): pixel centers, zoom anchors, relative panning, aspect fitting, reset and precision limits.
3. Add typed render settings, flat frame buffers, statistics, tiles and deterministic scoped-thread rendering (`src/render.rs`, `tests/render.rs`). Compare every alternate path with serial output.
4. Add RGB palettes, interpolation, cyclic shifts, gamma, histogram equalization and supersampling (`src/palette.rs`, `tests/palette.rs`). Preserve black interiors and validate configuration.
5. Implement PPM and PNG export (`src/export.rs`, `tests/export.rs`) with checked dimensions and real checksums; independently decode a generated PNG.
6. Replace the monolithic binary with tested options, batch output and interactive navigation (`src/cli.rs`, `src/main.rs`, `tests/cli.rs`). Non-TTY invocation must finish, not block on input. Cover exit codes and I/O errors.
7. Provide named views, executable examples, benchmarks, shader hints and usage/numerics documentation. Add CI and final quality gates.

For each behavior: add an assertion against the intended public API, run `cargo test` and observe the missing behavior, implement, run `cargo fmt && cargo test`, then commit a coherent change. Documentation-only commits are checked against the executable commands they describe.

## Validation

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features` and `cargo test --doc`
- `cargo build --release`
- Render ASCII, PPM and PNG; verify PNG pixels with an independent decoder.
- Compare serial and threaded hashes, exercise Julia, custom bounds and iteration limits.
- Audit commit count, nonempty diffs, weekday/evening dates, author/committer agreement and clean worktree; never push.

## History provenance

This development series was reconstructed in one maintenance session. Commit dates are intentionally assigned within March 2022–May 2023 at the repository owner's request; they are not evidence that these changes were developed at those historical times. Existing source commits remain available in the separately verified recovery bundle. No remote history is changed by this local work.
