// Uniforms
@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

// Texture Bindings
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

// Vertex Inputs

// struct Vertex {
//     @location(0) position: vec3<f32>,
//     @location(1) color: vec3<f32>, // Color, or UVs, or Both
//     @location(2) uvs: vec2<f32>,
//     @location(3) normals: vec3<f32>, // Must have both normals & Lighting values
//     @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
// };

struct VertexColor {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexUv {
    @location(0) position: vec3<f32>,
    @location(2) uvs: vec2<f32>,
};

struct VertexColorUv {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
};

struct VertexColorLit {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>,
};

struct VertexUvLit {
    @location(0) position: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>,
};

struct VertexColorUvLit {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>,
};