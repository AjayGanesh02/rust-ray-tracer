mod ray;
mod vec3;
mod sphere;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use log::info;
use std::{fs, io};

use crate::{
    ray::Ray,
    vec3::{FormatColor, Point3, Vec3},
};

// IMAGE CONSTANTS

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
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
const CAMERA_CENTER: Point3 = glam::DVec3::ZERO;

// calculate vectors along the horizontal and down the vertical edges

const VIEWPORT_U: Vec3 = glam::DVec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VIEWPORT_V: Vec3 = glam::DVec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

const MAX_VALUE: u32 = 255;

fn main() -> io::Result<()> {
    let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        CAMERA_CENTER - DVec3::new(0., 0., FOCAL_LENGTH) - VIEWPORT_U / 2. - VIEWPORT_V / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64)
        .map(|(y, x)| {
            let pixel_center =
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - CAMERA_CENTER;

            let cam_cur_ray = Ray::new(CAMERA_CENTER, ray_direction);
            let cur_color = cam_cur_ray.color();
            cur_color.format_color()
        })
        .chunks(IMAGE_WIDTH as usize)
        .into_iter()
        .map(|chunk| chunk.into_iter().join(" "))
        .join("\n");

    info!("{pixels}");

    fs::write(
        "output.ppm",
        format!(
            "P3
{} {}
{}
{}",
            IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE, pixels
        ),
    )
}
