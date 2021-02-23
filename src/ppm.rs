use glam::Vec3A;
use indicatif::ProgressBar;

pub fn gen_ppm(img: Vec<Vec<Vec3A>>, n_x: u32, n_y: u32, n_s: u32) {
    println!("P3\n{} {}\n255", n_x, n_y);
    let bar = ProgressBar::new(n_y as u64);
    for (_y, row) in img.iter().enumerate() {
        for (_x, pixel) in row.iter().enumerate() {
            let r = pixel.x;
            let g = pixel.y;
            let b = pixel.z;

            // Apply Gamma Correction here, we've already divided by n_s
            // So we don't need to scale these values
            let r_corr = r.sqrt();
            let g_corr = g.sqrt();
            let b_corr = b.sqrt();

            println!(
                "{} {} {}",
                (clamp(r_corr, 0.0, 0.999) * 256.0) as u32,
                (clamp(g_corr, 0.0, 0.999) * 256.0) as u32,
                (clamp(b_corr, 0.0, 0.999) * 256.0) as u32
            );
        }
        bar.inc(1);
    }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
