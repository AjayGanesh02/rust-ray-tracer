mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use material::Material;

use crate::{hittable::Hittable, sphere::Sphere, vec3::Vec3};

fn main() -> Result<(), std::io::Error> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let material_ground = Material::Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    };
    let material_left = Material::Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
    };
    let material_right = Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
    };

    let ground = Sphere::new(Vec3::new(0., -100.5, -1.), 100., material_ground);
    let center = Sphere::new(Vec3::new(0., 0., -1.), 0.5, material_center);
    let left = Sphere::new(Vec3::new(-1., 0., -1.), 0.5, material_left);
    let right = Sphere::new(Vec3::new(1., 0., -1.), 0.5, material_right);

    world.push(Box::new(ground));
    world.push(Box::new(center));
    world.push(Box::new(left));
    world.push(Box::new(right));

    Camera::new().render_to_disk(world, "output.ppm")
}
