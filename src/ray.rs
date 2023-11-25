use crate::{
    hittable::Hittable,
    vec3::{Color, Point3, Vec3},
};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

pub struct ScatteredRay {
    pub attenuation: Vec3,
    pub scattered: Ray,
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
            if let Some(ScatteredRay {
                attenuation,
                scattered,
            }) = hit_record.material.scatter(self, &hit_record)
            {
                return attenuation * scattered.color(world, depth - 1);
            }
            return Color::ZERO;
        }

        let unit_direction: Vec3 = self.direction.normalize();
        let a: f64 = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0).lerp(Color::new(0.5, 0.7, 1.0), a)
    }
}
