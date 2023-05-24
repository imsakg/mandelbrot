use mandelbrot::{GpuParams, RenderSettings, Viewport};
#[test]
fn gpu_precision_guard_uses_shader_f32_operations_after_rounding_bounds() {
    let s = RenderSettings {
        width: 3,
        height: 1,
        viewport: Viewport::new(100.0, 100.00002384185791, -1.0, 1.0).unwrap(),
        ..Default::default()
    };
    assert!(GpuParams::from_settings(&s).is_err());
}
