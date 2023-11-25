use crate::{
    sphere::Sphere,
    vec3::{Color, Point3, Vec3},
};

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
        let sphere = Sphere {
            center: Vec3::new(0., 0., -1.),
            radius: 0.5,
        };
        let t = hit_sphere(&sphere, &self);
        if t > 0.0 {
            let n = (self.at(t) - sphere.center).normalize();
            return 0.5 * (n + 1.);
        }

        let unit_direction: Vec3 = self.direction.normalize();
        let a: f64 = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0).lerp(Color::new(0.5, 0.7, 1.0), a)
    }
}

fn hit_sphere(sphere: &Sphere, ray: &Ray) -> f64 {
    let oc = ray.origin - sphere.center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - sphere.radius.powi(2);
    let discriminant = b.powi(2) - 4. * a * c;

    if discriminant < 0. {
        return -1.;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}
