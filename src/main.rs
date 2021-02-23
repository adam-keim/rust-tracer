use glam::Vec3A;
use indicatif::{ProgressBar, ProgressIterator};
use ray::Ray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod hittable;
mod ppm;
mod ray;
mod sphere;
fn main() {
    //Image Config
    let aspect_ratio = 16.0 / 9.0;
    let n_x: u32 = 1500; //This is where we change resolution
    let n_y = (n_x as f32 / aspect_ratio) as u32;

    // Camera Config
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3A::new(0.0, 0.0, 0.0);
    let horizontal = Vec3A::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3A::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3A::new(0.0, 0.0, focal_length);

    // for j in (0..n_y).rev().progress() {
    //     for i in 0..n_x {
    //         let u = i as f32 / n_x as f32;
    //         let v = j as f32 / n_y as f32;
    //         let r = Ray::new(
    //             origin,
    //             lower_left_corner + u * horizontal + v * vertical - origin,
    //         );
    //         let pixel_color = ray_color(r);
    //         write_color(pixel_color);
    //     }
    // }

    let bar = ProgressBar::new(n_y as u64);
    let scene: Vec<Vec<Vec3A>> = (0..n_y)
        .into_par_iter()
        .map(|y_rev| {
            let y: f32 = n_y as f32 - y_rev as f32 - 1.0;
            let row: Vec<Vec3A> = (0..n_x)
                .into_par_iter()
                .map(|x| {
                    let u = x as f32 / n_x as f32;
                    let v = y as f32 / n_y as f32;
                    let r = Ray::new(
                        origin,
                        lower_left_corner + u * horizontal + v * vertical - origin,
                    );
                    ray_color(r)
                })
                .collect();
            bar.inc(1);
            row
        })
        .collect();

    ppm::gen_ppm(scene, n_x, n_y);
}

fn ray_color(r: Ray) -> Vec3A {
    let t = hit_sphere(Vec3A::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let N = (r.at(t) - Vec3A::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Vec3A::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
    }
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3A::new(1.0, 1.0, 1.0) + t * Vec3A::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vec3A, radius: f32, ray: &Ray) -> f32 {
    let oc: Vec3A = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - radius * radius;
    let disc = half_b * half_b - a * c;

    if disc < 0.0 {
        -1.0
    } else {
        (-half_b - disc.sqrt()) / a
    }
}

fn write_color(color: Vec3A) {
    println!(
        "{} {} {}",
        (color.x * 255.99) as u32,
        (color.y * 255.99) as u32,
        (color.z * 255.99) as u32
    );
}
