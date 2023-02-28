// Reference compute pass. A host must upload the 48-byte GpuParams buffer,
// allocate width*height vec2<u32> results, dispatch 8x8 workgroups and read back.
struct Params {
    bounds: vec4<f32>,
    constant: vec2<f32>,
    size: vec2<u32>,
    iterations: u32,
    variant: u32,
    power: u32,
    radius_sqr: f32,
};
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read_write> result: array<vec2<u32>>;

fn multiply(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x*b.x-a.y*b.y, a.x*b.y+a.y*b.x);
}
fn power(z: vec2<f32>, exponent: u32) -> vec2<f32> {
    var value=vec2<f32>(1.0,0.0);
    for (var i=0u; i<exponent; i=i+1u) { value=multiply(value,z); }
    return value;
}
@compute @workgroup_size(8,8,1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x>=params.size.x || id.y>=params.size.y) { return; }
    let uv=(vec2<f32>(id.xy)+vec2<f32>(0.5))/vec2<f32>(params.size);
    let point=params.bounds.xz+(params.bounds.yw-params.bounds.xz)*uv;
    var c=point;
    var z=vec2<f32>(0.0);
    if (params.variant==1u) { c=params.constant; z=point; }
    var count=params.iterations;
    var escaped=0u;
    for (var i=1u; i<=params.iterations; i=i+1u) {
        if (params.variant==2u) { z=abs(z); }
        if (params.variant==3u) { z=power(z,params.power)+c; }
        else { z=multiply(z,z)+c; }
        if (!(dot(z,z)<=params.radius_sqr)) { count=i; escaped=1u; break; }
    }
    result[id.y*params.size.x+id.x]=vec2<u32>(count,escaped);
}
