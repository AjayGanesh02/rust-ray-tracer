use crate::{
    hittable::HitRecord,
    ray::{Ray, ScatteredRay},
    vec3::{random_unit_vector, Color, Vec3},
};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
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
            Self::Dielectric {
                index_of_refraction,
            } => {
                let refraction_ratio = if hit_record.front_face {
                    index_of_refraction.recip()
                } else {
                    *index_of_refraction
                };
                let unit_direction = ray.direction.normalize();
                let cos_theta = (-unit_direction)
                    .dot(hit_record.normal)
                    .clamp(f64::NEG_INFINITY, 1.0);
                let sin_theta = (1. - cos_theta.powi(2)).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refract {
                    // Must Reflect
                    reflect(unit_direction, hit_record.normal)
                } else {
                    // Can Refract
                    refract(unit_direction, hit_record.normal, refraction_ratio)
                };

                Some(ScatteredRay {
                    attenuation: Color::splat(1.),
                    scattered: Ray::new(hit_record.point, direction),
                })
            }
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

pub fn refract(uv: Vec3, n: Vec3, refraction_ratio: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n).clamp(f64::NEG_INFINITY, 1.);
    let r_out_perp = refraction_ratio * (uv + cos_theta * n);
    let r_out_para = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -1. * n;
    r_out_perp + r_out_para
}
