use glam::DVec3;
use rand::random;

pub type Vec3 = DVec3;

pub type Point3 = Vec3;

pub type Color = Vec3;

pub trait FormatColor {
    fn format_color(&self, samples_per_pixel: u32) -> String;
}

impl FormatColor for Color {
    fn format_color(&self, samples_per_pixel: u32) -> String {
        let scaled = *self * (samples_per_pixel as f64).recip();
        let gamma_corrected = scaled.powf(0.5);
        let pixel_color = (gamma_corrected.clamp(Vec3::ZERO, Vec3::splat(0.999)) * 256.).as_ivec3();
        format!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z)
    }
}

fn random_vec3() -> Vec3 {
    Vec3::new(random(), random(), random())
}

fn randrange_vec3(min: f64, max: f64) -> Vec3 {
    min + random_vec3() * (max - min)
}

fn random_in_unit_sphere_vec3() -> Vec3 {
    loop {
        let p = randrange_vec3(-1., 1.);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere_vec3().normalize()
}

// pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
//     let on_unit_sphere = random_unit_vector();
//     if on_unit_sphere.dot(normal) > 0.0 {
//         return on_unit_sphere;
//     } else {
//         return -on_unit_sphere;
//     }
// }

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}
