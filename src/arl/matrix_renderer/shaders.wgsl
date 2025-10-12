struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>
};

@group(0) @binding(0)
var<uniform> camera: mat4x4<f32>;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) data: vec4<f32>,
    @location(3) data1: vec4<f32>,
    @location(4) data2: vec4<f32>,
    @location(5) data3: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = color;
    let mat = mat4x4<f32>(data, data1, data2, data3);
    out.clip_position = camera * mat * vec4<f32>(position, 1.0);
    return out;
}
 

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
 