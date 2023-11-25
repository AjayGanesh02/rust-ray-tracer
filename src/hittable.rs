use std::ops::Range;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

fn calc_face_normal(ray: &Ray, normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.direction.dot(normal) < 0.0;
    let normal = if front_face { normal } else { -normal };
    (front_face, normal)
}

impl HitRecord {
    pub fn with_face_normal(
        point: Point3,
        outward_normal: Vec3,
        t: f64,
        material: &Material,
        ray: &Ray,
    ) -> Self {
        let (front_face, normal) = calc_face_normal(ray, outward_normal);
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material: material.clone(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        // closest seen is used as endpoint in interval, and starts at interval.end
        let (_closest, hit_record) = self.iter().fold((interval.end, None), |acc, item| {
            if let Some(temp_rec) = item.hit(ray, interval.start..acc.0) {
                (temp_rec.t, Some(temp_rec))
            } else {
                acc
            }
        });

        hit_record
    }
}
