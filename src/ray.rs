use crate::{
    hittable::Hittable,
    vec3::{random_on_hemisphere, random_unit_vector, Color, Point3, Vec3},
};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn color<T>(&self, world: &T, depth: u32) -> Color
    where
        T: Hittable,
    {
        if depth <= 0 {
            return Color::ZERO;
        }
        if let Some(hit_record) = world.hit(self, 0.001..f64::INFINITY) {
            let direction = hit_record.normal + random_unit_vector();
            return 0.1 * Ray::new(hit_record.point, direction).color(world, depth - 1);
        }

        let unit_direction: Vec3 = self.direction.normalize();
        let a: f64 = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0).lerp(Color::new(0.5, 0.7, 1.0), a)
    }
}
