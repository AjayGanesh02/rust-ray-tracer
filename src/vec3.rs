use glam::DVec3;

pub type Vec3 = DVec3;

pub type Point3 = Vec3;

pub type Color = Vec3;

pub trait FormatColor {
    fn format_color(&self, samples_per_pixel: u32) -> String;
}

impl FormatColor for Color {
    fn format_color(&self, samples_per_pixel: u32) -> String {
        let scaled = *self * (samples_per_pixel as f64).recip();
        let pixel_color = (scaled.clamp(Vec3::ZERO, Vec3::splat(0.999)) * 255.999).as_ivec3();
        format!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z)
    }
}
