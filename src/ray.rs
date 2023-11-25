use glam::DVec3;

use crate::vec3::{Color, Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Color {
        let unit_direction: Vec3 = self.direction.normalize();
        let a: f64 = 0.5 * (unit_direction.y + 1.0);
        return Color::new(1.0, 1.0, 1.0).lerp(Color::new(0.5, 0.7, 1.0), a);
    }
}
