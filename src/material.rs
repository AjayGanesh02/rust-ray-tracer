use crate::{
    hittable::HitRecord,
    ray::{Ray, ScatteredRay},
    vec3::{random_unit_vector, reflect, Color, Vec3},
};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + random_unit_vector();

                // catch degenerate scatter direction
                if scatter_direction.abs_diff_eq(Vec3::new(0., 0., 0.), 1e-8) {
                    scatter_direction = hit_record.normal;
                }

                Some(ScatteredRay {
                    attenuation: *albedo,
                    scattered: Ray {
                        origin: hit_record.point,
                        direction: scatter_direction,
                    },
                })
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = reflect(ray.direction.normalize(), hit_record.normal);
                let scattered = reflected + fuzz.clamp(0., 1.) * random_unit_vector();
                Some(ScatteredRay {
                    attenuation: *albedo,
                    scattered: Ray::new(hit_record.point, scattered),
                })
            }
        }
    }
}
