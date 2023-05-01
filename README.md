# Mandelbrot

A terminal fractal explorer and reusable Rust rendering library. The original demo is preserved below; the current renderer also supports Julia sets, Burning Ship and powers 2–8 of the Multibrot family.

![Original terminal demonstration](intro.gif)

## Run

Install Rust 1.88 or a newer stable toolchain, then:

```sh
cargo run --release -- --width 80 --height 24
cargo run --release -- --interactive --width 80 --height 22
cargo run --release -- --help
```

No flag means one ASCII frame and exit, so pipes and automated runs never wait for keyboard input. Interactive mode requires both stdin and stdout to be terminals. It uses a temporary alternate screen and restores the cursor on normal exit or returned errors.

Interactive keys: `wasd` pan by 10% of the visible span, `k`/`+` zoom in, `j`/`-` zoom out, `[`/`]` halve/double the iteration budget, `r` reset bounds, `u` undo, `y` redo, `p` select the export palette, `q`/Escape quit. Unknown keys do nothing. Undo tracks viewports, not every color/iteration setting.

## Export

```sh
cargo run --release -- --width 800 --height 600 --iterations 800 --palette fire --threads 4 --output mandelbrot.png
cargo run --release -- --preset julia --width 640 --height 480 --samples 2 --palette viridis --output julia.ppm
cargo run --release -- --preset seahorse --width 640 --height 480 --histogram --gamma 2.2 --output seahorse.png
```

`.png` and `.ppm` select the format (extensions are case-insensitive). PNG is RGB8 with valid CRC/zlib data but deliberately uncompressed DEFLATE blocks, so files are larger than optimized PNGs. Exports use an owned sibling temporary and atomic hard-link publication: **existing files are never overwritten**. Filesystems without hard-link support return an error instead of silently weakening this guarantee. Parent directories must already exist.

Palettes: `ocean`, `fire`, `gray`, and `viridis` (a five-stop approximation, not the official lookup table). `--cycles` and `--shift` repeat/offset the palette; `--no-smooth` retains integer bands. Black denotes samples still bounded at the iteration cap, not a proof of membership. `--samples 2` averages a 2×2 subpixel grid; the maximum is 4×4.

## Explore specific regions

```sh
cargo run --release -- --bounds -0.80,-0.70,0.05,0.15 --zoom 2 --pan 0.1,-0.1 --iterations 1500
cargo run --release -- --julia -0.8,0.156
cargo run --release -- --variant ship
cargo run --release -- --power 3 --radius 4
```

Bounds are `xmin,xmax,ymin,ymax`, sampled at pixel centers. Image rows increase imaginary coordinates (screen-down). Viewport transforms and `--preset` apply in argument order. Zoom factors greater than one zoom in; positive pan values move right/down. Image aspect ratios are not silently fitted: use the library's `fit_aspect` for square pixels or account for terminal cell geometry yourself.

Presets: `overview`, `seahorse`, `elephant`, `julia`, `ship`. Set dimensions explicitly for image work; default dimensions remain terminal-sized (100×30). Caps are 16 million rendered samples, 10 million iterations per orbit, 64 CPU workers and four samples per axis. Supersampling counts toward the sample cap; very large budgets can still be expensive. Collapsed f64 pixel centers return an explicit zoom-precision error.

## Library and development

```sh
cargo run --release --example render -- example.png
cargo run --release --example benchmark -- --width 320 --height 240 --threads 4
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo test --doc
```

The library separates orbit kernels, viewport navigation, orbit frames, RGB palettes and image encoding. `render` is the serial correctness oracle; `render_parallel` produces the same row-major samples with scoped workers. `render_with_progress` supports row callbacks and cancellation. `Options::recipe` creates a portable scene description without machine-local paths.

- [Numerical conventions and limits](docs/NUMERICS.md)
- [GPU porting reference](docs/GPU.md): WGSL and checked uniform packing, **not a GPU backend**
- [Implementation and history provenance](docs/IMPLEMENTATION.md)

Commit dates in the reconstructed development series were intentionally reassigned to March 2022–May 2023; they are not contemporaneous development evidence. This local reconstruction does not publish or rewrite any remote by itself.
