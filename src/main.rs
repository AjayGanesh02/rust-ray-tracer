use log::info;
mod vec3;

use crate::vec3::Vec3;
fn main() {
    let v1 = Vec3::newz();
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    println!("{:?}", v1 + v2);
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        info!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let r = (i as f32) / ((image_width - 1) as f32);
            let g = (j as f32) / ((image_height - 1) as f32);
            let b: f32 = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    info!("\rDone.                 \n")
}
