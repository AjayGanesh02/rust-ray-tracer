use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use log::info;
use std::{fs, io};

pub trait WriteColor {
    fn write_color(&self) -> String;
}

impl WriteColor for glam::DVec3 {
    fn write_color(&self) -> String {
        format!(
            "{} {} {}",
            (self.x * 255.999) as i32,
            (self.y * 255.999) as i32,
            (self.z * 255.999) as i32
        )
    }
}

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    const MAX_VALUE: u32 = 255;

    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64)
        .map(|(y, x)| {
            let color = DVec3::new(
                (x as f64) / ((IMAGE_WIDTH - 1) as f64), // r
                (y as f64) / ((IMAGE_WIDTH - 1) as f64), // g
                0.0,                                     // b
            );

            color.write_color()
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
{IMAGE_WIDTH} {IMAGE_HEIGHT}
{MAX_VALUE}
{pixels}"
        ),
    )
}
