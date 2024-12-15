// Uniforms
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Camera {
    position: vec4<f32>,
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
}

// Texture Bindings
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

// Light Bindings
@group(2) @binding(0)
var<uniform> lights: array<Light, 16>;

struct Light {
    color_intensity: vec4<f32>,
    position_range: vec4<f32>,
    direction_angle: vec4<f32>,
}

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
}

// Vertex Inputs

// struct VertexIn {
//     @location(0) position: vec3<f32>,
//     @location(1) color: vec3<f32>, // Color, or UVs, or Both
//     @location(2) uvs: vec2<f32>,
//     @location(3) normals: vec3<f32>, // Must have both normals & Lighting values
//     @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
// };

// struct VertexOut {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(1) color: vec3<f32>,
//     @location(2) uvs: vec2<f32>,
//     @location(3) terms: vec4<f32>, // Lighting Terms
//     @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
// };

// Vertex Color
struct VertexColorIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexColorOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_color(
    model: VertexColorIn,
    instance: InstanceInput,
) -> VertexColorOut {
    var out: VertexColorOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    out.color = model.color;
    out.clip_position = camera.proj * camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_color(in: VertexColorOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

// Vertex UVs
struct VertexUvIn {
    @location(0) position: vec3<f32>,
    @location(2) uvs: vec2<f32>,
};

struct VertexUvOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uvs: vec2<f32>,
};

@vertex
fn vs_uv(
    model: VertexUvIn,
    instance: InstanceInput,
) -> VertexUvOut {
    var out: VertexUvOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    out.uvs = model.uvs;
    out.clip_position = camera.proj * camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_uv(in: VertexUvOut) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uvs);
}

// Vertex Color + UVs
struct VertexColorUvIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
};

struct VertexColorUvOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
};

@vertex
fn vs_color_uv(
    model: VertexColorUvIn,
    instance: InstanceInput,
) -> VertexColorUvOut {
    var out: VertexColorUvOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    out.color = model.color;
    out.uvs = model.uvs;
    out.clip_position = camera.proj * camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_color_uv(in: VertexColorUvOut) -> @location(0) vec4<f32> {
    var texel = textureSample(t_diffuse, s_diffuse, in.uvs);
    return vec4<f32>(in.color * texel.rgb, 1.0);
}

// Vertex Color + Lighting
struct VertexColorLitIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>,
};

struct VertexColorLitOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(3) terms: vec4<f32>,
    @location(4) lighting: vec3<f32>,
};

@vertex
fn vs_color_lit(
    model: VertexColorLitIn,
    instance: InstanceInput,
) -> VertexColorLitOut {
    // TODO: Write This Shader!
    var out: VertexColorLitOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    out.color = model.color;
    out.clip_position = camera.proj * camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_color_lit(in: VertexColorLitOut) -> @location(0) vec4<f32> {
    // TODO: Write This Shader!
    return vec4<f32>(in.color, 1.0);
}

// Vertex UV + Lighting
struct VertexUvLitIn {
    @location(0) position: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>,
};

struct VertexUvLitOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) terms: vec4<f32>,
    @location(4) lighting: vec3<f32>,
};

@vertex
fn vs_uv_lit(
    model: VertexUvLitIn,
    instance: InstanceInput,
) -> VertexUvLitOut {
    // TODO: Write This Shader!
    var out: VertexUvLitOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    out.uvs = model.uvs;
    out.clip_position = camera.proj * camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_uv_lit(in: VertexUvLitOut) -> @location(0) vec4<f32> {
    // TODO: Write This Shader!
    return textureSample(t_diffuse, s_diffuse, in.uvs);
}

// Vertex Color + UV + Lighting
struct VertexColorUvLitIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>,
};

struct VertexColorUvLitOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) terms: vec4<f32>,
    @location(4) lighting: vec3<f32>,
};

// void vertex() {
//     // Step 1: Transform the vertex and normals to world space
//     vec3 world_position = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
//     vec3 world_normal = normalize((MODEL_MATRIX * vec4(NORMAL, 0.0)).xyz);

// 	// Calculate view direction in world space
//     vec3 viewDir = normalize(CAMERA_POSITION_WORLD - world_position);
//     // Calculate light direction in world space
//     vec3 lightDir = normalize(light_pos - world_position);
// 	// Compute the half-vector for the Cook-Torrance model
//     vec3 halfDir = normalize(viewDir + lightDir);

//     //// Relevant dot products
//     frag_NdotV = max(dot(world_normal, viewDir), 0.0);
//     frag_NdotL = max(dot(world_normal, lightDir), 0.0);
//     frag_NdotH = max(dot(world_normal, halfDir), 0.0);
//     frag_VdotH = max(dot(viewDir, halfDir), 0.0);
// 	frag_F0 = mix(0.04, 1.0, mat_metallic);  // 0.04 for dielectric, 1.0 for conductor (fully metallic)


//     VERTEX = (VIEW_MATRIX * vec4(world_position, 1.0)).xyz;
// 	NORMAL = normalize((MODELVIEW_MATRIX * vec4(NORMAL, 0.0)).xyz);
// }

@vertex
fn vs_color_uv_lit(
    model: VertexColorUvLitIn,
    instance: InstanceInput,
) -> VertexColorUvLitOut {
    // TODO: Write This Shader!
    var out: VertexColorUvLitOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    out.color = model.color;
    out.uvs = model.uvs;
    out.clip_position = camera.proj * camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_color_uv_lit(in: VertexColorUvLitOut) -> @location(0) vec4<f32> {
    // TODO: Write This Shader!
    let texel = textureSample(t_diffuse, s_diffuse, in.uvs);
    return vec4<f32>(in.color * texel.rgb, 1.0);
}

// Lighting Parts
const PI = radians(180.0);

struct LightingTerms {
    n_dot_v: f32,
    n_dot_l: f32,
    n_dot_h: f32,
    v_dot_h: f32,
}

fn fresnel_schlick(cos_theta: f32, f_0: f32) -> f32 {
    let cos_theta_clamped = clamp(cos_theta, 0.001, 1.0); // Avoid exactly zero or negative values
    return f_0 + (1.0 - f_0) * pow(1.0 - cos_theta_clamped, 5.0);
}

// GGX / Trowbridge-Reitz Normal Distribution Function
fn d_ggx(n_dot_h: f32, roughness: f32) -> f32 {
    let alpha = roughness * roughness;
    let alpha2 = alpha * alpha;
    let denom = (n_dot_h * n_dot_h) * (alpha2 - 1.0) + 1.0;
    return alpha2 / (PI * denom * denom);
}

// Geometry function using Smith's Schlick-GGX
fn g_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    let k = (roughness + 1.0) * (roughness + 1.0) / 8.0;
    return n_dot_v / (n_dot_v * (1.0 - k) + k);
}

// Cook-Torrance BRDF
fn cook_torrance_specular(
    light_color: vec3<f32>,
    n_dot_l: f32,
    n_dot_v: f32,
    n_dot_h: f32,
    v_dot_h: f32,
    roughness: f32,
    f_0: f32,
) -> vec3<f32> {
    // Fresnel term (Schlick approximation)
    let f = fresnel_schlick(v_dot_h, f_0);

    // Normal distribution function (GGX)
    let d = d_ggx(n_dot_h, roughness);

    // Geometry function (Smith's Schlick-GGX)
    let g = g_schlick_ggx(n_dot_v, roughness) * g_schlick_ggx(n_dot_l, roughness);

    // Cook-Torrance denominator
    let denom = 4.0 * n_dot_v * n_dot_l + 0.001; // Avoid division by zero

    // Final Cook-Torrance specular term
    return (d * g * f) / denom * light_color;
}