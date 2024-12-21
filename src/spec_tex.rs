use glam::Vec3;
use image::GrayImage;
use rayon::prelude::*;
use std::f32::consts::PI;

// Helper functions
fn f_unreal(f_0: f32, v_dot_h: f32) -> f32 {
    let exponent = ((-5.55473 * v_dot_h) - 6.98316) * v_dot_h;
    return f_0 + ((1.0 - f_0) * 2.0f32.powf(exponent));
}

fn normal_approx(shininess: f32) -> f32 {
    return (0.0397436 * shininess) + 0.0856832;
}

fn generate_normal(step: usize, max: usize) -> Vec3 {
    // Calculate the angle for this step
    let max = max - 1;
    let angle = (PI / 2.0) * (1.0 - step as f32 / max as f32);

    // Compute the Y and Z components
    let y = angle.sin(); // Cosine for the Y component
    let z = angle.cos(); // Sine for the Z component

    // X is always 0
    Vec3::new(0.0, y, z).normalize()
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

// Specular BRDF computation
fn precompute_brdf(
    texture_resolution: (usize, usize),
    metallic: f32,
    hemisphere_samples: &[Vec3],
) -> Vec<u8> {
    let (resolution_u, resolution_v) = texture_resolution;
    let mut brdf_texture = vec![0; resolution_u * resolution_v]; // Greyscale

    let f0 = lerp(0.04, 1.0, metallic);

    let view_dir = Vec3::new(0.0, 0.0, 1.0);

    for v in 0..resolution_v {
        let roughness = (v as f32) / (resolution_v as f32 - 1.0); // V corresponds to roughness
        let shininess = 2048.0f32.powf(1.0 - roughness); // MAX SHININESS

        for u in 0..resolution_u {
            let normal = generate_normal(u, resolution_u);
            let n_dot_v = normal.dot(view_dir);

            // Use stratified hemisphere sampling
            let mut specular_sum = 0.0;
            for light_dir in hemisphere_samples {
                let half_dir = (light_dir + view_dir).normalize();
                let n_dot_l = Vec3::dot(normal, *light_dir).max(0.0);
                let n_dot_h = Vec3::dot(normal, half_dir).max(0.0);
                let v_dot_h = Vec3::dot(view_dir, half_dir).max(0.0);

                // let diffuse_color = 1.0 - metallic;
                // let diffuse = (diffuse_color / PI) * (1.0 - f0);

                let fresnel = f_unreal(f0, v_dot_h);
                let top = fresnel * f0 * n_dot_h.powf(shininess);
                let bottom = n_dot_l.max(n_dot_v);
                let specular = normal_approx(shininess) * (top / bottom);
                specular_sum += specular;
            }

            // Average the samples and store in the texture
            let sample_count = hemisphere_samples.len();
            let specular_avg = specular_sum / sample_count as f32;
            let idx = v * resolution_u + u;

            let specular_avg = specular_avg * u8::MAX as f32;
            brdf_texture[idx] = specular_avg as u8;
        }
    }

    brdf_texture
}

fn stratified_sample_hemisphere(samples_sqrt: usize) -> Vec<Vec3> {
    let mut directions = Vec::new();
    let inv_samples = 1.0 / samples_sqrt as f32;

    for i in 0..samples_sqrt {
        for j in 0..samples_sqrt {
            // Stratified sample coordinates
            let u = (i as f32 + 0.5) * inv_samples; // Latitude
            let v = (j as f32 + 0.5) * inv_samples; // Longitude

            // Convert to spherical coordinates
            let theta = (1.0 - u).acos(); // Map [0, 1] to [0, π/2]
            let phi = 2.0 * PI * v; // Map [0, 1] to [0, 2π]

            // Convert to Cartesian coordinates
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            directions.push(
                Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta).normalize(),
            );
        }
    }

    directions
}

// Generates a 3d texture with the following coordinates:
// u x: NdotV
// v, y: Shininess
// w: F0 (scalar) or color
pub fn generate_texture() {
    let texture_resolution = (64, 64); // Resolution for U (NdotV), V (roughness), W (metallic)
    let depth = 64;

    let sample_count = 64;
    let samples = stratified_sample_hemisphere(sample_count);

    let mut images: Vec<(u32, Vec<u8>)> = (0..depth)
        .par_bridge()
        .map(|d| {
            let normalized_d = d as f32 / (depth - 1) as f32; // Normalize d to be between 0 and 1
            (
                d,
                precompute_brdf(texture_resolution, normalized_d, &samples),
            )
        })
        .collect();

    images.sort_by_key(|e| e.0);

    let width = texture_resolution.0 as u32;
    let height = texture_resolution.1 as u32;

    let mut out_image = GrayImage::new(width * depth, height);
    out_image
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let x = x as usize;
            let y = y as usize;
            let image = x / width as usize;
            let x = x % width as usize;

            pixel.0 = [images[image].1[y * width as usize + x]];
        });

    out_image.save("ambient_brdf.png").unwrap();
}
