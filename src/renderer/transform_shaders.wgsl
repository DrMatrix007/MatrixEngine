struct TransformRaw {
    position: vec3<f32>,
    rotation: vec3<f32>,
    scale: vec3<f32>,
}


@group(0) @binding(0) var<storage,read> inputBuffer: TransformRaw; 
@group(1) @binding(0) var<storage,read_write> outputBuffer: mat4x4<f32>; 


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

fn translation_matrix(translation: vec3<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(translation, 1.0),
    );
}

fn scale_matrix(scale: vec3<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4<f32>(scale.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, scale.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, scale.z, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

@compute @workgroup_size(32)
fn cs_main(@builtin(global_invocation_id) id:vec3<u32>) {
    let transform: TransformRaw = inputBuffer;

    // Create the rotation matrix (Z * Y * X)
    let rotation = rotation_matrix_z(transform.rotation.z) * rotation_matrix_y(transform.rotation.y) * rotation_matrix_x(transform.rotation.x);

    let translation = translation_matrix(transform.position);
    let scale = scale_matrix(transform.scale);

    let modelMatrix = translation * rotation * scale;

    outputBuffer = modelMatrix;
}