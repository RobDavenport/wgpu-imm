struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    //@location(1) color: vec3<f32>, // Color, or UVs, or Both
    @location(2) uvs: vec2<f32>,
    // @location(3) normals: vec3<f32>, // Must have both normals & Lighting values
    // @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = vec3<f32>(model.uvs, 0.0);
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
 