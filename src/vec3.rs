use glam::DVec3;

pub type Vec3 = DVec3;

pub type Point3 = Vec3;

pub type Color = Vec3;

pub trait FormatColor {
    fn format_color(&self) -> String;
}

impl FormatColor for Color {
    fn format_color(&self) -> String {
        format!(
            "{} {} {}",
            (self.x * 255.999) as i32,
            (self.y * 255.999) as i32,
            (self.z * 255.999) as i32
        )
    }
}