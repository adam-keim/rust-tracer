use glam::Vec3A;
use indicatif::ProgressBar;

pub fn gen_ppm(img: Vec<Vec<Vec3A>>, n_x: u32, n_y: u32) {
    println!("P3\n{} {}\n255", n_x, n_y);
    let bar = ProgressBar::new(n_y as u64);
    for (y, row) in img.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            println!(
                "{} {} {}",
                (pixel.x * 255.99) as u32,
                (pixel.y * 255.99) as u32,
                (pixel.z * 255.99) as u32
            );
        }
        bar.inc(1);
    }
}
