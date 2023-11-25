use std::fs;

use indicatif::ProgressIterator;
use itertools::Itertools;
use log::info;
use rand::{self, random};

use crate::{
    hittable::Hittable,
    ray::Ray,
    vec3::{FormatColor, Point3, Vec3},
};

// IMAGE CONSTANTS
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 960;
// calculate the image height, and ensure that it is at least 1
const IMAGE_HEIGHT_UNFIXED: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const IMAGE_HEIGHT: u32 = if IMAGE_HEIGHT_UNFIXED >= 1 {
    IMAGE_HEIGHT_UNFIXED
} else {
    1
};

// CAMERA

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
// less than 1 for viewport width is ok
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const CAMERA_CENTER: Point3 = Vec3::ZERO;

// calculate vectors along the horizontal and down the vertical edges

const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

const MAX_VALUE: u32 = 255;
const SAMPLES_PER_PIXEL: u32 = 10;

const MAX_DEPTH: u32 = 10;

pub struct Camera {
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
        let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;

        let viewport_upper_left =
            CAMERA_CENTER - Vec3::new(0., 0., FOCAL_LENGTH) - VIEWPORT_U / 2. - VIEWPORT_V / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }

    pub fn render(&self, world: Vec<Box<dyn Hittable>>) -> String {
        let pixels = (0..IMAGE_HEIGHT)
            .cartesian_product(0..IMAGE_WIDTH)
            .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64)
            .map(|(y, x)| {
                (0..SAMPLES_PER_PIXEL)
                    .into_iter()
                    .map(|_| self.get_ray(x, y).color(&world, MAX_DEPTH))
                    .sum::<Vec3>()
                    .format_color(SAMPLES_PER_PIXEL)
            })
            .chunks(IMAGE_WIDTH as usize)
            .into_iter()
            .map(|chunk| chunk.into_iter().join(" "))
            .join("\n");

        info!("{pixels}");
        format!(
            "P3
{} {}
{}
{}",
            IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE, pixels
        )
    }

    pub fn render_to_disk(
        &self,
        world: Vec<Box<dyn Hittable>>,
        path: &str,
    ) -> Result<(), std::io::Error> {
        fs::write(path, (*self).render(world))
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        Ray::new(CAMERA_CENTER, pixel_sample - CAMERA_CENTER)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}
