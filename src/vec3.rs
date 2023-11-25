use glam::DVec3;

pub type Vec3 = DVec3;

pub type Point3 = Vec3;

pub type Color = Vec3;

pub trait FormatColor {
    fn format_color(&self) -> String;
}

impl FormatColor for Color {
    fn format_color(&self) -> String {
        let pixel_color = (*self * 255.999).as_ivec3();
        format!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z)
    }
}
