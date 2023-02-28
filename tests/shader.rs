#[test]
fn reference_compute_shader_parses_and_validates() {
    let module = naga::front::wgsl::parse_str(include_str!("../shaders/fractal.wgsl")).unwrap();
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    let entry = &module.entry_points[0];
    assert_eq!(entry.workgroup_size, [8, 8, 1]);
}
