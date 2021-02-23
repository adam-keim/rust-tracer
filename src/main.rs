use std::f32::INFINITY;

use glam::Vec3A;
use hittable::{Hittable, HittableList};
use indicatif::ProgressBar;
use ray::Ray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sphere::Sphere;

mod camera;
mod hittable;
mod ppm;
mod ray;
mod sphere;

fn main() {
    //Image Config
    let aspect_ratio = 16.0 / 9.0;
    let n_x: u32 = 400; //This is where we change resolution
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

    // Scene Config
    let mut list = HittableList::new();
    list.add_hittable(Sphere::new(Vec3A::new(0.0, 0.0, -1.0), 0.5));
    list.add_hittable(Sphere::new(Vec3A::new(0.0, -100.5, -1.0), 100.0));

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
                    ray_color(&r, &list)
                })
                .collect();
            bar.inc(1);
            row
        })
        .collect();

    ppm::gen_ppm(scene, n_x, n_y);
}

fn ray_color(r: &Ray, world: &HittableList) -> Vec3A {
    if let Some(hr) = world.hit(r, 0.0, INFINITY) {
        0.5 * (hr.norm + Vec3A::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3A::new(1.0, 1.0, 1.0) + t * Vec3A::new(0.5, 0.7, 1.0)
    }
}
