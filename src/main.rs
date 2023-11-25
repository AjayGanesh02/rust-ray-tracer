mod ray;
mod vec3;

use indicatif::ProgressIterator;
use itertools::Itertools;
use log::info;
use std::{fs, io};

use crate::vec3::{FormatColor, Vec3};

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: u32 = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    // calculate the image height, and ensure that it is at least 1
    let mut image_height: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    image_height = if image_height > 1 { image_height } else { 1 };

    // calculate viewport measurements - less than 1 is ok
    const VIEWPORT_HEIGHT: f64 = 2.0;
    let viewport_width = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / image_height as f64);

    const MAX_VALUE: u32 = 255;
    // let test = Ray{}

    let pixels = (0..image_height)
        .cartesian_product(0..IMAGE_WIDTH)
        .progress_count(image_height as u64 * IMAGE_WIDTH as u64)
        .map(|(y, x)| {
            let color = Vec3::new(
                (x as f64) / ((IMAGE_WIDTH - 1) as f64),  // r
                (y as f64) / ((image_height - 1) as f64), // g
                0.0,                                      // b
            );

            color.format_color()
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
            IMAGE_WIDTH, image_height, MAX_VALUE, pixels
        ),
    )
}
