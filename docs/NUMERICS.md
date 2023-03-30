# Numerical conventions

## Escape-time contract

Mandelbrot samples begin with `z₀=0`, `c=sample`, then update `z←z²+c`. Julia samples begin with `z₀=sample` and a fixed user constant. Burning Ship applies `abs` to both components before squaring; Multibrot uses `z←zᵖ+c` for integer `p` from 2 through 8.

An iteration counts one completed update. The bailout comparison is strict: `|z|² > R²`; landing exactly on the circle is not an escape. The configured radius must be finite, at least 2, and squareable. Julia internally expands the radius to `max(R, |c|)` so bounded fixed points outside the default radius are not incorrectly discarded. Its constants must also have a finite squareable magnitude. Nonfinite polynomial results from overflow count as escaped; NaN must not silently look bounded.

`Escape.escaped` disambiguates a last-update escape from an orbit still bounded at the budget. `iterations == max_iterations` alone is not a membership test. Budgets are in 1..=10,000,000 in validated APIs. The low-level `escape_point` also permits a zero budget, returning a bounded zero-update result; prefer `iterate` for untrusted inputs.

The main cardioid and period-two bulb tests are Mandelbrot-only shortcuts. They use strict inequalities and return the full iteration budget, with a placeholder `norm_sqr=0` for analytically skipped interiors. `Statistics.iteration_sum` therefore counts represented iteration budgets, **not actual CPU work**.

## Smooth colors and distances

For an escaped orbit with magnitude `r` and degree `p`, the continuous count is `n + 1 - log(log(r))/log(p)`. Invalid logarithms or overflow fall back to the integer count. Colors divide the result by the budget, apply cycles and a wrapped phase shift, then interpolate palette stops. Escaped-only histogram equalization uses the midpoint of each discrete iteration bin's CDF instead; interior samples never influence this distribution.

Derivative iteration uses `d←p*z^(p−1)*d + 1` for parameter-space sets, starting at zero; Julia uses `d←2*z*d`, starting at one. The exterior estimate is `r*log(r)/|d|`. It is an approximation, not a signed distance, exact boundary classifier or robust antialiasing guarantee. Burning Ship is nonanalytic, so it returns no derivative distance. Nonpositive, overflowing or nonfinite estimates return `None`.

Palette interpolation is sRGB display-space. Gamma correction applies `channel^(1/gamma)` after interpolation; it does not alter the explicitly selected interior RGB. Regular-grid supersampling averages those displayed RGB colors, not linear-light radiance. These are pragmatic image choices, not physically based colorimetry.

## Geometry and precision

Bounds are strictly ordered finite rectangles. Pixel `(x,y)` samples normalized coordinates `((x+0.5)/width,(y+0.5)/height)`; image rows increase imaginary values. A 1×1 render samples the viewport center. Zooms preserve either the center or a normalized anchor, and pans use fractions of the current spans.

Adjacent pixel centers must remain strictly increasing in f64. Validation checks the actual sampled centers, not only whether adding a step changes a bound. This guard rejects unresolved zooms rather than inventing extra detail; it does not provide arbitrary precision. GPU references use f32 and have a separate stricter resolution guard.

Serial and parallel CPU paths share the same sample kernel and ordering. Thread count changes scheduling, not geometry, palette mapping or escape semantics. There are no unsafe SIMD intrinsics or claims of GPU speedups. Floating-point behavior may still differ across architectures/toolchains at sensitive boundaries; exact mathematical membership cannot be inferred from finite iteration images.

## Memory and runtime

`MAX_PIXELS` caps intermediate orbit samples at 16,000,000, including supersampling. Each orbit stores counts, magnitude and an optional distance; RGB and PNG intermediate buffers add to peak memory. Allocation failures in the orbit buffer are returned as errors; standard container allocations elsewhere can still abort on allocator exhaustion. The image cap is not a promise of low memory use.

Worst-case work scales with width × height × samples² × iteration budget. These independent caps do not form a wall-time bound. Cancel a library render through its row callback; the CLI has no mid-orbit interruption mechanism. Use small previews before increasing dimensions or budgets.
