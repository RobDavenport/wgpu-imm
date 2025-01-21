// Consts
const MAX_LIGHTS = 4;
const PI = radians(180.0);
const INV_PI = 1.0 / PI;
const MAX_SHININESS = 2048.0;
const LIGHT_FALLOFF = 2.0;

// -- Per Frame Bindings --
// Camera
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Camera {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
    ortho: mat4x4<f32>,
    pos: vec4<f32>,
}

@group(0) @binding(1)
var<storage> views: array<mat4x4<f32>>;

@group(0) @binding(2)
var<storage> positions: array<vec4<f32>>;

@group(0) @binding(3)
var<storage> projections: array<mat4x4<f32>>;

// Light Bindings
@group(0) @binding(4)
var<uniform> lights: array<Light, MAX_LIGHTS>;

// Environent Map Bindings
@group(0) @binding(5)
var t_env: texture_cube<f32>;
@group(0) @binding(6)
var s_env: sampler;
@group(0) @binding(7)
var<uniform> env_color_strength: vec4<f32>;
// -- End Per Frame Bindings --

// Texture Bindings
@group(1) @binding(0)
var t_albedo: texture_2d<f32>;
@group(1) @binding(1)
var s_albedo: sampler;

// Matcaps
@group(2) @binding(0)
var t_matcap: texture_2d<f32>;

@group(2) @binding(1)
var s_matcap: sampler;

@group(2) @binding(2)
var t_matcap2: texture_2d<f32>;

@group(2) @binding(3)
var t_matcap3: texture_2d<f32>;

@group(2) @binding(4)
var t_matcap4: texture_2d<f32>;

struct PushConstants {
    blend_modes: u32,
}

var<push_constant> push_constants: PushConstants;

struct Light {
    color_max_angle: vec4<f32>,
    position_range: vec4<f32>,
    direction_min_angle: vec4<f32>,
}

struct InstanceInput {
    @location(6) model_matrix_0: vec4<f32>,
    @location(7) model_matrix_1: vec4<f32>,
    @location(8) model_matrix_2: vec4<f32>,
    @location(9) model_matrix_3: vec4<f32>,
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
//     @location(3) normals: vec3<f32>,
//     @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
//     @location(0) view_pos: vec3<f32>,
//     @location(5) world_reflection: vec3<f32>,
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
    return textureSample(t_albedo, s_albedo, in.uvs);
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
    let texel = textureSample(t_albedo, s_albedo, in.uvs).rgb;
    return vec4<f32>(in.color * texel, 1.0);
}

// Vertex Color + Lighting
struct VertexColorLitIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
};

struct VertexColorLitOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
    @location(0) view_pos: vec3<f32>,
    @location(5) world_reflection: vec3<f32>,
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

    let view_position = views[0] * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = projections[0] * view_position;
    out.color = model.color;
    out.normals = normalize((views[0] * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.view_pos = view_position.xyz;
    out.lighting = model.lighting;

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);
    let world_normal = normalize(model_matrix * vec4<f32>(model.normals, 0.0));
    let incoming = normalize(positions[0] - world_position);
    out.world_reflection = reflect(incoming, world_normal).xyz;

    return out;
}

@fragment
fn fs_color_lit(in: VertexColorLitOut) -> @location(0) vec4<f32> {
    let frag_color = in.color;

    let output_color = calculate_lighting(
        frag_color,
        in.view_pos,
        in.normals,
        in.world_reflection,
        in.lighting
    );
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
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
    @location(0) view_pos: vec3<f32>,
    @location(5) world_reflection: vec3<f32>,
};

@vertex
fn vs_uv_lit(
    model: VertexUvLitIn,
    instance: InstanceInput,
) -> VertexUvLitOut {
    var out: VertexUvLitOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);

    out.uvs = model.uvs;
    out.clip_position = camera.proj * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.lighting = model.lighting;

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);
    let world_normal = normalize(model_matrix * vec4<f32>(model.normals, 0.0));
    let incoming = normalize(camera.pos - world_position);
    out.world_reflection = reflect(incoming, world_normal).xyz;

    return out;
}

@fragment
fn fs_uv_lit(in: VertexUvLitOut) -> @location(0) vec4<f32> {
    let frag_color = textureSample(t_albedo, s_albedo, in.uvs).rgb;

    let output_color = calculate_lighting(
        frag_color,
        in.view_pos,
        in.normals,
        in.world_reflection,
        in.lighting
    );
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
    @location(3) normals: vec3<f32>,
    @location(4) lighting: vec3<f32>, // Metallic, Roughness, Emissive
    @location(0) view_pos: vec3<f32>,
    @location(5) world_reflection: vec3<f32>,
};

@vertex
fn vs_color_uv_lit(
    model: VertexColorUvLitIn,
    instance: InstanceInput,
) -> VertexColorUvLitOut {
    var out: VertexColorUvLitOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);

    out.color = model.color;
    out.uvs = model.uvs;
    out.clip_position = camera.proj * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.lighting = model.lighting;

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);
    let world_normal = normalize(model_matrix * vec4<f32>(model.normals, 0.0));
    let incoming = normalize(camera.pos - world_position);
    out.world_reflection = reflect(incoming, world_normal).xyz;

    return out;
}

@fragment
fn fs_color_uv_lit(in: VertexColorUvLitOut) -> @location(0) vec4<f32> {
    let texel = textureSample(t_albedo, s_albedo, in.uvs).rgb;
    let frag_color = in.color * texel.rgb;

    let output_color = calculate_lighting(
        frag_color,
        in.view_pos,
        in.normals,
        in.world_reflection,
        in.lighting
    );
    return vec4<f32>(output_color, 1.0);
}

// Lighting Parts

// Source:
// https://cdn2.unrealengine.com/Resources/files/2013SiggraphPresentationsNotes-26915738.pdf
fn f_unreal(f_0: vec3<f32>, v_dot_h: f32) -> vec3<f32> {
    let exponent = ((-5.55473 * v_dot_h) - 6.98316) * v_dot_h;
    return f_0 + ((1.0 - f_0) * pow(2.0, exponent));
}

fn roughness_to_shininess(roughness: f32) -> f32 {
    let shininess = pow(MAX_SHININESS, 1.0 - roughness);
    return shininess;
}

// Source:
// https://research.tri-ace.com/Data/course_note_practical_implementation_at_triace.pdf
fn normalize_shininess(shininess: f32) -> f32 {
    return (0.0397436 * shininess) + 0.0856832;
}

fn calculate_light(
    albedo: vec3<f32>,
    metallic: f32,
    roughness: f32,
    view_position: vec3<f32>,
    view_normal: vec3<f32>,
    light: Light,
) -> vec3<f32> {
    var terms: vec4<f32>;
    var light_dir: vec3<f32>;

    // View direction in view space
    let view_dir = normalize(-view_position);
    let n_dot_v = max(dot(view_normal, view_dir), 0.0);

    var attenuation = 1.0;

    // Identify light type
    if light.position_range.w <= 0.0 {
        // Global Light (Ambient or Directional)

        // No Direction, Ambient Light
        if all(light.direction_min_angle.xyz == vec3<f32>(0.0)) {
            let light_color = light.color_max_angle.rgb;
            return tri_ace_ambient(albedo, light_color, metallic, n_dot_v);
        } else {
            // Directional Light
            light_dir = normalize(-light.direction_min_angle.xyz);
        }
    } else {
        // Positional Light (Point or Spot)
        light_dir = normalize(light.position_range.xyz - view_position);
        attenuation = calculate_attenuation(light.position_range, view_position);

        // Early out for spotlights
        if light.direction_min_angle.w > 0.0 {
            // Spot light
            let spot_factor = dot(light_dir, normalize(-light.direction_min_angle.xyz));
            // Check if within the spotlight cone
            if spot_factor < light.color_max_angle.w {
                // Fragment outside the cone, no light contribution
                return vec3<f32>(0.0);
            } else {
                // Exponential falloff based on the spot factor
                let angle_range = light.direction_min_angle.w - light.color_max_angle.w;
                let normalized_spot_factor = (spot_factor - light.color_max_angle.w) / angle_range;
                // Adjust exponent for sharper/softer falloff
                let falloff = pow(normalized_spot_factor, LIGHT_FALLOFF);
                attenuation *= falloff;
            }
        }
    }

    // Half vector calculation
    let half_vec = normalize(view_dir + light_dir);
    let n_dot_l = max(dot(view_normal, light_dir), 0.0);
    let n_dot_h = max(dot(view_normal, half_vec), 0.0);
    let v_dot_h = max(dot(view_dir, half_vec), 0.0);

    terms = vec4<f32>(n_dot_v, n_dot_l, n_dot_h, v_dot_h);

    let light_color = light.color_max_angle.rgb * attenuation;
    return tri_ace_directional(albedo, light_color, metallic, roughness, terms);
}

// Based off of the version below
fn tri_ace_environment(
    albedo: vec3<f32>,
    reflection_color: vec3<f32>,
    metallic: f32,
    n_dot_v: f32,
) -> vec3<f32> {
    // Set Up Colors
    let diffuse_color = (1.0 - metallic) * albedo; // Non-metallic materials will use diffuse color
    let f_0 = mix(vec3(0.04), albedo, metallic); // This becomes the specular color
    let env_color = env_color_strength.xyz;
    let env_strength = env_color_strength.w;

    // Specular Term
    let f = f_unreal(f_0, n_dot_v);
    let specular = (f * f_0) * reflection_color;
    let diffuse = (diffuse_color * env_color * INV_PI) * (1.0 - f_0);

    return (diffuse + specular) * env_strength;
}

// Based off of the version below
fn tri_ace_ambient(
    albedo: vec3<f32>,
    light_color: vec3<f32>,
    metallic: f32,
    n_dot_v: f32,
) -> vec3<f32> {
    // Set Up Colors
    let diffuse_color = (1.0 - metallic) * albedo; // Non-metallic materials will use diffuse color
    let f_0 = mix(vec3(0.04), albedo, metallic); // This becomes the specular color

    // Specular Term
    let f = f_unreal(f_0, n_dot_v);
    let specular = (f * f_0) * INV_PI; // Divide it by PI, since it's "diffuse"
    let diffuse = (diffuse_color * INV_PI) * (1.0 - f_0);

    return (specular + diffuse) * light_color;
}

// Shader Implementation Reference:
// https://research.tri-ace.com/Data/course_note_practical_implementation_at_triace.pdf
fn tri_ace_directional(
    texel_color: vec3<f32>,
    light_color: vec3<f32>,
    metallic: f32,
    roughness: f32,
    terms: vec4<f32>,
) -> vec3<f32> {
    let n_dot_v = terms[0];
    let n_dot_l = terms[1];
    let n_dot_h = terms[2];
    let v_dot_h = terms[3];

    let shininess = roughness_to_shininess(roughness);

    // Set Up Colors
    let diffuse_color = (1.0 - metallic) * texel_color * light_color; // Non-metallic materials will use diffuse color
    let f_0 = mix(vec3(0.04), texel_color, metallic); // This becomes the specular color

    // Diffuse Term
    let diffuse = (diffuse_color * INV_PI) * (1.0 - f_0);

    // Specular Term
    let f = f_unreal(f_0, v_dot_h);
    let top = f * f_0 * pow(n_dot_h, shininess);
    let bot = max(n_dot_l, n_dot_v);
    let specular = normalize_shininess(shininess) * (top / bot) * light_color;

    return (diffuse + specular) * n_dot_l;
}

// Generates a random vec3 from a seed
fn random_vec3(seed: vec3<f32>) -> vec3<f32> {
    return fract(sin(seed * vec3<f32>(12.9898, 45.3467, 78.5643)) * 43758.5453) * 2.0 - 1.0;
}

fn get_reflection(reflect_dir: vec3<f32>, roughness: f32) -> vec3<f32> {
    // Calculate alpha as roughness2 / PI
    let alpha = roughness * roughness * INV_PI;

    // Add jitter based on alpha, more rough = higher distance
    let jitter = random_vec3(reflect_dir) * alpha;
    let p_a = normalize(reflect_dir + jitter);
    let p_b = normalize(reflect_dir - jitter);

    // Sample two jittered points
    let c_a = textureSample(t_env, s_env, p_a).rgb;
    let c_b = textureSample(t_env, s_env, p_b).rgb;

    return (c_a + c_b) * 0.5; // Average the sum
}

fn calculate_attenuation(light_position_range: vec4<f32>, fragment_pos: vec3<f32>) -> f32 {
    let distance = length(light_position_range.xyz - fragment_pos) / light_position_range.w;
    let clamped = clamp(distance, 0.0, 1.0);
    return pow(1.0 - clamped, LIGHT_FALLOFF);
}

fn calculate_lighting(
    albedo: vec3<f32>,
    view_pos: vec3<f32>,
    normal: vec3<f32>,
    world_reflection: vec3<f32>,
    lighting: vec3<f32>,
) -> vec3<f32> {
    // Extract lighting values
    let metallic = lighting.r;
    let roughness = lighting.g;
    let emissive = lighting.b;

    var output_color = albedo * emissive; // Emissive Factor

    // Get the environment color
    let n_normal = normalize(normal);
    var reflection = normalize(world_reflection);
    reflection.y = -reflection.y;
    let reflection_color = get_reflection(reflection, roughness);

    let n_dot_v = dot(n_normal, normalize(-view_pos));

    // Apply environment color
    output_color += tri_ace_environment(albedo, reflection_color, metallic, n_dot_v);

    // Apply all lights
    for (var i = 0; i < MAX_LIGHTS; i++) {
        let l = calculate_light(albedo, metallic, roughness, view_pos, n_normal, lights[i]);

        output_color += l;
    }

    return output_color;
}

@vertex
fn vs_quad_2d(
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
    out.clip_position = camera.ortho * model_matrix * vec4<f32>(model.position, 1.0);

    return out;
}

fn matcap_uv(view: vec3<f32>, normal: vec3<f32>) -> vec2<f32> {
  let inv_depth = 1.0 / (1.0 + view.z);
  let proj_factor = -view.x * view.y * inv_depth;
  let basis1 = vec3(1.0 - view.x * view.x * inv_depth, proj_factor, -view.x);
  let basis2 = vec3(proj_factor, 1.0 - view.y * view.y * inv_depth, -view.y);
  let matcap_uv = vec2(dot(basis1, normal), dot(basis2, normal));

  return matcap_uv * vec2(0.5, -0.5) + 0.5;
}

struct VertexMatcapIn {
    @location(0) position: vec3<f32>,
    @location(3) normals: vec3<f32>,
}

struct VertexMatcapOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) view_pos: vec3<f32>,
    @location(3) normals: vec3<f32>,
}

@vertex
fn vs_matcap(
    model: VertexMatcapIn,
    instance: InstanceInput,
) -> VertexMatcapOut {
    var out: VertexMatcapOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = camera.proj * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);

    return out;
}

@fragment
fn fs_matcap(in: VertexMatcapOut) -> @location(0) vec4<f32> {
    let normal = normalize(in.normals);
    let view = normalize(-in.view_pos);
    // TODO: Fix this so the first one is always replace
    let matcap = get_matcaps(vec3<f32>(0.0), view, normal);
    return vec4<f32>(matcap, 1.0);
}

struct VertexMatcapColorIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(3) normals: vec3<f32>,
};

struct VertexMatcapColorOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(3) normals: vec3<f32>,
    @location(0) view_pos: vec3<f32>,
};

@vertex
fn vs_matcap_color(
    model: VertexMatcapColorIn,
    instance: InstanceInput,
) -> VertexMatcapColorOut {
    var out: VertexMatcapColorOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = camera.proj * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.color = model.color;

    return out;
}

@fragment
fn fs_matcap_color(in: VertexMatcapColorOut) -> @location(0) vec4<f32> {
    let normal = normalize(in.normals);
    let view = normalize(-in.view_pos);
    let color = get_matcaps(in.color, view, normal);
    return vec4<f32>(color, 1.0);
}

struct VertexMatcapUvIn {
    @location(0) position: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
};

struct VertexMatcapUvOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
    @location(0) view_pos: vec3<f32>,
};

@vertex
fn vs_matcap_uv(
    model: VertexMatcapUvIn,
    instance: InstanceInput,
) -> VertexMatcapUvOut {
    var out: VertexMatcapUvOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = camera.proj * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.uvs = model.uvs;

    return out;
}

@fragment
fn fs_matcap_uv(in: VertexMatcapUvOut) -> @location(0) vec4<f32> {
    let normal = normalize(in.normals);
    let view = normalize(-in.view_pos);
    let texel = textureSample(t_albedo, s_albedo, in.uvs).rgb;
    let color = get_matcaps(texel, view, normal);

    return vec4<f32>(color, 1.0);
}

struct VertexMatcapColorUvIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
};

struct VertexMatcapColorUvOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) normals: vec3<f32>,
    @location(0) view_pos: vec3<f32>,
};

@vertex
fn vs_matcap_color_uv(
    model: VertexMatcapColorUvIn,
    instance: InstanceInput,
) -> VertexMatcapColorUvOut {
    var out: VertexMatcapColorUvOut;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let view_position = camera.view * model_matrix * vec4<f32>(model.position, 1.0);

    out.clip_position = camera.proj * view_position;
    out.view_pos = view_position.xyz;
    out.normals = normalize((camera.view * model_matrix * vec4<f32>(model.normals, 0.0)).xyz);
    out.uvs = model.uvs;
    out.color = model.color;

    return out;
}

@fragment
fn fs_matcap_color_uv(in: VertexMatcapColorUvOut) -> @location(0) vec4<f32> {
    let normal = normalize(in.normals);
    let view = normalize(-in.view_pos);
    let texel = textureSample(t_albedo, s_albedo, in.uvs).rgb;
    let color = get_matcaps(texel * in.color, view, normal);

    return vec4<f32>(color, 1.0);
}

fn get_matcaps(start_color: vec3<f32>, view: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let matcap_uv = matcap_uv(view, normal);

    var texels: array<vec3<f32>, 4>;
    texels[0] = textureSample(t_matcap, s_matcap, matcap_uv).rgb;
    texels[1] = textureSample(t_matcap2, s_matcap, matcap_uv).rgb;
    texels[2] = textureSample(t_matcap3, s_matcap, matcap_uv).rgb;
    texels[3] = textureSample(t_matcap4, s_matcap, matcap_uv).rgb;

    return blend_layers(start_color, &texels);
}

fn blend_layers(start_color: vec3<f32>, texels: ptr<function, array<vec3<f32>, 4>>) -> vec3<f32> {
    var out = start_color;        

    for (var i = 0u; i < 4; i++) {
        let texel = (*texels)[i];
        let byte = (push_constants.blend_modes >> (i * 8u)) & 0xFF;
        switch byte {
            case 1u: {
                out = texel;
            }
            case 2u: {
                out = blend_add(out, texel);
            }
            case 3u: {
                out = blend_screen(out, texel);
            }
            case 4u: {
                out = blend_color_dodge(out, texel);
            }
            case 5u: {
                out = blend_subtract(out, texel);
            }
            case 6u: {
                out = blend_multiply(out, texel);
            }
            case 7u: {
                out = blend_color_burn(out, texel);
            }
            case 8u: {
                out = blend_overlay(out, texel);
            }
            default: {
                return out;
            }
        }
    }

    return out;
}

// // Normal (Replace): Simply uses the blend color
// fn blend_normal(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
//     return blend;
// }

// Add: base + blend
fn blend_add(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return clamp(base + blend, vec3<f32>(0.0), vec3<f32>(1.0));
}

// Screen: 1 - (1 - base) * (1 - blend)
fn blend_screen(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return 1.0 - (1.0 - base) * (1.0 - blend);
}

// Color Dodge: base / (1 - blend)
fn blend_color_dodge(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return clamp(base / (1.0 - blend), vec3<f32>(0.0), vec3<f32>(1.0));
}

// Subtract: base - blend
fn blend_subtract(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return clamp(base - blend, vec3<f32>(0.0), vec3<f32>(1.0));
}

// Multiply: base * blend
fn blend_multiply(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return base * blend;
}

// Color Burn: 1 - ((1 - base) / blend)
fn blend_color_burn(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return clamp(1.0 - ((1.0 - base) / blend), vec3<f32>(0.0), vec3<f32>(1.0));
}

// Overlay: Combines Multiply and Screen based on base
fn blend_overlay(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    let multiplier = 2.0 * base * blend;
    let screen = 1.0 - 2.0 * (1.0 - base) * (1.0 - blend);
    let mask = step(vec3<f32>(0.5), base); // Creates a vec3 mask based on base
    return mix(multiplier, screen, mask);
}
