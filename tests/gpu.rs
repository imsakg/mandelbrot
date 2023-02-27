use mandelbrot::{GpuParams, RenderSettings, Viewport};
#[test]
fn gpu_uniforms_are_explicitly_packed_and_f32_precision_is_guarded() {
    let p = GpuParams::from_settings(&RenderSettings::default()).unwrap();
    assert_eq!(p.to_le_bytes().len(), 48);
    assert_eq!(p.workgroups(), (100, 75));
    let s = RenderSettings {
        width: 800,
        height: 600,
        viewport: Viewport::new(1.0, 1.0 + 1e-8, -1.0, 1.0).unwrap(),
        ..Default::default()
    };
    assert!(GpuParams::from_settings(&s).is_err());
}
