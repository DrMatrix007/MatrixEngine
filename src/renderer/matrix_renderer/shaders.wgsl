// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceTransform {
    @location(5) mat1: vec4<f32>,
    @location(6) mat2: vec4<f32>,
    @location(7) mat3: vec4<f32>,
    @location(8) mat4: vec4<f32>,
}

fn into_mat(m:InstanceTransform) -> mat4x4<f32> {
    return mat4x4<f32>(
        m.mat1,
        m.mat2,
        m.mat3,
        m.mat4
    );
}


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> camera_proj: mat4x4<f32>;

@vertex
fn v_main(
    model: VertexInput,
    instance: InstanceTransform,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera_proj * into_mat(instance) * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}