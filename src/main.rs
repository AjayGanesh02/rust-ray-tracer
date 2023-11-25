use std::{fs, io};

use itertools::Itertools;
fn main() -> io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    const MAX_VALUE: u32 = 255;

    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .map(|(y, x)| {
            let r = (x as f32) / ((IMAGE_WIDTH - 1) as f32);
            let g = (y as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let b: f32 = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            format!("{ir} {ig} {ib}")
        })
        .chunks(IMAGE_WIDTH as usize)
        .into_iter()
        .map(|chunk| chunk.into_iter().join(" "))
        .join("\n");

    println!("{pixels}");

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
