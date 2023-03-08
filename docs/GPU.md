# GPU porting notes

The shipping renderer uses f64 CPU arithmetic, serially or with `--threads`. **There is no GPU backend or acceleration switch.** The WGSL shader is a validated porting reference; tests parse and validate it with Naga, but do not execute it on a GPU.

## Host contract

1. Validate `RenderSettings` and call `GpuParams::from_settings`. This rejects adjacent-pixel collapse and out-of-range f32 constants/radii; passing does not guarantee CPU/GPU orbit equality near a boundary.
2. Upload `GpuParams::to_le_bytes()` as a 48-byte uniform buffer at group 0, binding 0. Layout: four f32 bounds `[xmin,xmax,ymin,ymax]`, two f32 Julia constants, two u32 dimensions, then u32 iterations, variant and power, and f32 squared radius. All words are little-endian; align the binding to the device's uniform-buffer requirements.
3. Allocate `width * height * 8` bytes as a writable storage buffer at binding 1. Each row-major `vec2<u32>` stores completed iterations and an escaped flag. Bounded samples retain the budget and flag zero.
4. Dispatch `GpuParams::workgroups()` workgroups of 8×8 invocations; excess edge invocations return without writing.
5. Read back or colorize in a separate pass. The reference omits smooth coloring, derivative distances, analytic cardioid shortcuts and histogram reduction.

## Precision and performance

- CPU f64 is the correctness oracle. Most portable WGSL implementations provide f32 only; deep zooms lose detail earlier. Compare escape masks with tolerance at boundaries, not fabricated speedup percentages.
- Mandelbrot and Julia orbits diverge in iteration count within each workgroup. Tile planning is a scheduling hint, not proof of occupancy or acceleration.
- Keep orbit buffers resident when changing palettes. Smooth and distance values require additional output storage and careful overflow behavior.
- Measure upload, dispatch, synchronization and readback separately, with identical images, budgets, presets and palettes. Small terminal frames can be faster on CPU because dispatch overhead dominates.
- Validate on at least two device backends before calling an accelerated path supported. Query limits, preserve the CPU fallback, and report device creation/validation failures rather than silently claiming GPU execution.

`cargo test --test shader --test gpu` verifies the source-level shader and host packing contract. Hardware execution and GPU/CPU parity remain deliberately unimplemented.
