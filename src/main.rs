mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;

use crate::{hittable::Hittable, sphere::Sphere, vec3::Vec3};

fn main() -> Result<(), std::io::Error> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    Camera::new().render_to_disk(world, "output.ppm")
}
