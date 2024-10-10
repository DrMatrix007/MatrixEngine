@group(0) @binding(0) var<storage,read_write> buffer: array<mat4x4<f32>>; 


fn rotation_matrix_x(angle: f32) -> mat4x4<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, c, s, 0.0),
        vec4<f32>(0.0, -s, c, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

fn rotation_matrix_y(angle: f32) -> mat4x4<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return mat4x4<f32>(
        vec4<f32>(c, 0.0, -s, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(s, 0.0, c, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

fn rotation_matrix_z(angle: f32) -> mat4x4<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return mat4x4<f32>(
        vec4<f32>(c, s, 0.0, 0.0),
        vec4<f32>(-s, c, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

fn translation_matrix(translation: vec4<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4<f32>(1., 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(translation.xyz, 1.0),
    );
}

fn scale_matrix(scale: vec4<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4<f32>(scale.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, scale.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, scale.z, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

@compute @workgroup_size(256,1,1)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let transform = buffer[id.x];


    let rotation = rotation_matrix_z(transform[1].z) * rotation_matrix_y(transform[1].y) * rotation_matrix_x(transform[1].x);

    // let translation = translation_matrix(transform[1].xyzw + vec4(f32(id.x)));
    // Create the translation matrix
    let translation = translation_matrix(transform[0]);

    // Combine translation and rotation-scale
    let modelMatrix = rotation ;

    // Store the result in the outputBuffer
    buffer[id.x] = translation * rotation;
    buffer[id.x] = buffer[id.x];
}