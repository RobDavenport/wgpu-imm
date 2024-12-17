// Consts
const MAX_LIGHTS = 8;

// Uniforms
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Camera {
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
var<uniform> lights: array<Light, MAX_LIGHTS>;

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
//     @location(3) lighting: vec3<f32>, // Metallic, Roughness, Emissive
//     @location(9) terms_0: vec4<f32>, // Lighting Terms
//     @location(10) terms_1: vec4<f32>, // Lighting Terms
//     @location(11) terms_2: vec4<f32>, // Lighting Terms
//     @location(12) terms_3: vec4<f32>, // Lighting Terms
//     @location(13) terms_4: vec4<f32>, // Lighting Terms
//     @location(14) terms_5: vec4<f32>, // Lighting Terms
//     @location(15) terms_6: vec4<f32>, // Lighting Terms
//     @location(16) terms_7: vec4<f32>, // Lighting Terms
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
    var texel = textureSample(t_diffuse, s_diffuse, in.uvs).rgb;
    return vec4<f32>(in.color * texel, 1.0);
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
    @location(3) lighting: vec3<f32>, // Metallic, Roughness, Emissive
    @location(9) view_pos: vec3<f32>,
    @location(10) terms_0: vec4<f32>, // Lighting Terms
    @location(11) terms_1: vec4<f32>,
    @location(12) terms_2: vec4<f32>,
    @location(13) terms_3: vec4<f32>,
    @location(14) terms_4: vec4<f32>,
    @location(15) terms_5: vec4<f32>,
    @location(16) terms_6: vec4<f32>,
    @location(17) terms_7: vec4<f32>,
};

@vertex
fn vs_color_lit(
    model: VertexColorLitIn,
    instance: InstanceInput,
) -> VertexColorLitOut {
    var out: VertexColorLitOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    // Transform position and normal to view space
    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    let view_normal = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);

    let terms = calculate_lighting_terms(view_position.xyz, view_normal.xyz);
    out.terms_0 = terms[0];
    out.terms_1 = terms[1];
    out.terms_2 = terms[2];
    out.terms_3 = terms[3];
    out.terms_4 = terms[4];
    out.terms_5 = terms[5];
    out.terms_6 = terms[6];
    out.terms_7 = terms[7];
    out.lighting = model.lighting;
    out.view_pos = view_position.xyz;

    out.color = model.color;
    out.clip_position = camera.proj * view_position;
    return out;
}

@fragment
fn fs_color_lit(in: VertexColorLitOut) -> @location(0) vec4<f32> {
    let frag_color = in.color;

    var terms: array<vec4<f32>, MAX_LIGHTS>;
    terms[0] = in.terms_0;
    terms[1] = in.terms_1;
    terms[2] = in.terms_2;
    terms[3] = in.terms_3;
    terms[4] = in.terms_4;
    terms[5] = in.terms_5;
    terms[6] = in.terms_6;
    terms[7] = in.terms_7;
    let output_color = calculate_lighting_color(&terms, in.view_pos, frag_color, in.lighting);

    return vec4<f32>(output_color, 1.0);
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
    @location(3) lighting: vec3<f32>, // Metallic, Roughness, Emissive
    @location(9) view_pos: vec3<f32>,
    @location(10) terms_0: vec4<f32>, // Lighting Terms
    @location(11) terms_1: vec4<f32>,
    @location(12) terms_2: vec4<f32>,
    @location(13) terms_3: vec4<f32>,
    @location(14) terms_4: vec4<f32>,
    @location(15) terms_5: vec4<f32>,
    @location(16) terms_6: vec4<f32>,
    @location(17) terms_7: vec4<f32>,
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

    // Transform position and normal to view space
    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    let view_normal = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);

    let terms = calculate_lighting_terms(view_position.xyz, view_normal.xyz);
    out.terms_0 = terms[0];
    out.terms_1 = terms[1];
    out.terms_2 = terms[2];
    out.terms_3 = terms[3];
    out.terms_4 = terms[4];
    out.terms_5 = terms[5];
    out.terms_6 = terms[6];
    out.terms_7 = terms[7];
    out.lighting = model.lighting;
    out.view_pos = view_position.xyz;

    out.uvs = model.uvs;
    out.clip_position = camera.proj * view_position;
    return out;
}

@fragment
fn fs_uv_lit(in: VertexUvLitOut) -> @location(0) vec4<f32> {
    let frag_color = textureSample(t_diffuse, s_diffuse, in.uvs).rgb;

    var terms: array<vec4<f32>, MAX_LIGHTS>;
    terms[0] = in.terms_0;
    terms[1] = in.terms_1;
    terms[2] = in.terms_2;
    terms[3] = in.terms_3;
    terms[4] = in.terms_4;
    terms[5] = in.terms_5;
    terms[6] = in.terms_6;
    terms[7] = in.terms_7;
    let output_color = calculate_lighting_color(&terms, in.view_pos, frag_color, in.lighting);

    return vec4<f32>(output_color, 1.0);
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
    @location(3) lighting: vec3<f32>, // Metallic, Roughness, Emissive
    @location(9) view_pos: vec3<f32>,
    @location(10) terms_0: vec4<f32>, // Lighting Terms
    @location(11) terms_1: vec4<f32>,
    @location(12) terms_2: vec4<f32>,
    @location(13) terms_3: vec4<f32>,
    @location(14) terms_4: vec4<f32>,
    @location(15) terms_5: vec4<f32>,
    @location(16) terms_6: vec4<f32>,
    @location(17) terms_7: vec4<f32>,
};

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

    // Transform position and normal to view space
    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);
    let view_normal = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);

    let terms = calculate_lighting_terms(view_position.xyz, view_normal.xyz);
    out.terms_0 = terms[0];
    out.terms_1 = terms[1];
    out.terms_2 = terms[2];
    out.terms_3 = terms[3];
    out.terms_4 = terms[4];
    out.terms_5 = terms[5];
    out.terms_6 = terms[6];
    out.terms_7 = terms[7];
    out.lighting = model.lighting;
    out.view_pos = view_position.xyz;

    out.color = model.color;
    out.uvs = model.uvs;
    out.clip_position = camera.proj * view_position;
    return out;
}

@fragment
fn fs_color_uv_lit(in: VertexColorUvLitOut) -> @location(0) vec4<f32> {
    let texel = textureSample(t_diffuse, s_diffuse, in.uvs).rgb;
    let frag_color = in.color * texel.rgb;

    var terms: array<vec4<f32>, MAX_LIGHTS>;
    terms[0] = in.terms_0;
    terms[1] = in.terms_1;
    terms[2] = in.terms_2;
    terms[3] = in.terms_3;
    terms[4] = in.terms_4;
    terms[5] = in.terms_5;
    terms[6] = in.terms_6;
    terms[7] = in.terms_7;
    let output_color = calculate_lighting_color(&terms, in.view_pos, frag_color, in.lighting);

    return vec4<f32>(output_color, 1.0);
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

// Used in Vertex Shader
fn calculate_lighting_terms(view_position: vec3<f32>, view_normal: vec3<f32>) -> array<vec4<f32>, MAX_LIGHTS> {
    var terms: array<vec4<f32>, MAX_LIGHTS>;
    var light_dir: vec3<f32>;

    // View direction in view space
    let view_dir = normalize(-view_position.xyz);

    for (var i = 0; i < MAX_LIGHTS; i++) {
        let light = lights[i];

        // Identify light type
        if (light.position_range.w < 0.0) {
            // Global Light (Ambient or Directional)

            if (all(light.direction_angle.xyz == vec3<f32>(0.0))) {
                continue; // Ambient light -- Do nothing
            } else {
                // Directional Light
                light_dir = normalize(-light.direction_angle.xyz);
            }
        } else {
            // Positional Light (Point or Spot)
            light_dir = normalize(light.position_range.xyz - view_position.xyz);

            if (light.position_range.w > 0.0) {
                // Spot light
                let spot_factor = dot(light_dir, normalize(-light.direction_angle.xyz));
                if (spot_factor < cos(light.direction_angle.w)) {
                    continue; // Don't include this light in the calculation
                }
            }
        }

        // Half vector calculation
        let half_vec = normalize(view_dir + light_dir);

        let n_dot_v = max(dot(view_normal, view_dir), 0.0);
        let n_dot_l = max(dot(view_normal, light_dir), 0.0);
        let n_dot_h = max(dot(view_normal, half_vec), 0.0);
        let v_dot_h = max(dot(view_dir, half_vec), 0.0);

        terms[i] = vec4<f32>(n_dot_v, n_dot_l, n_dot_h, v_dot_h);
    }

    return terms;
}

// Used in Fragment Shader
fn calculate_lighting_color(
    terms: ptr<function, array<vec4<f32>, MAX_LIGHTS>>,
    frag_view_pos: vec3<f32>,
    frag_color: vec3<f32>, 
    lighting: vec3<f32>
) -> vec3<f32> {
    let metallic = lighting.r;
    let roughness = lighting.g;
    let emissive = lighting.b;
    let f_0 = mix(0.04, 1.0, metallic);

    var output_color = vec3<f32>(0.0);

    for (var i = 0; i < MAX_LIGHTS; i++) {
        let term_ref = (*terms)[i];
        let term = LightingTerms(term_ref.x, term_ref.y, term_ref.z, term_ref.w);
        let light = lights[i];
        let light_color = light.color_intensity.rgb * light.color_intensity.w;

        // Identify light type
        if (light.position_range.w < 0.0 && all(light.direction_angle.xyz == vec3<f32>(0.0))) {
            // Ambient Light
            output_color += frag_color * light_color;
        } else {
            // Directional, Point, or Spot Light
            let specular = cook_torrance_specular(light_color, term.n_dot_l, term.n_dot_v, term.n_dot_h, term.v_dot_h, roughness, f_0);
            let diffuse = (1.0 - metallic) * term.n_dot_l;
            let final_color = (1.0 - metallic) * diffuse * light_color + specular;
            let lit_color = frag_color * final_color;

            if (light.position_range.w >= 0.0) {
                // Spot or Point Light
                // TODO!
                // Calculate distance from light to this fragment
                // Do distance falloff
            }

            output_color += lit_color;
        }

    }

    return output_color;
}