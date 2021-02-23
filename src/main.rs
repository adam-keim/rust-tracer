use std::f32::INFINITY;

use camera::Camera;
use glam::Vec3A;
use hittable::{Hittable, HittableList};
use indicatif::ProgressBar;
use rand::Rng;
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
    let n_s: u32 = 100;
    let max_depth = 50;

    // Camera Config
    let camera = Camera::new(); //Modify Camera Parameters in camera.rs

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
                    let mut color_vector: Vec3A = Vec3A::new(0.0, 0.0, 0.0);
                    let mut rng = rand::thread_rng();

                    for _s in 0..n_s {
                        let u: f32 = (x as f32 + rand::random::<f32>()) / n_x as f32;
                        let v: f32 = (y as f32 + rand::random::<f32>()) / n_y as f32;
                        let r: Ray = camera.get_ray(u, v);
                        color_vector += ray_color(&r, &list, max_depth, &mut rng);
                    }
                    color_vector /= n_s as f32;
                    color_vector
                })
                .collect();
            bar.inc(1);
            row
        })
        .collect();

    ppm::gen_ppm(scene, n_x, n_y, n_s);
}

fn ray_color<R: Rng>(r: &Ray, world: &HittableList, depth: u32, rng: &mut R) -> Vec3A {
    if depth <= 0 {
        return Vec3A::new(0.0, 0.0, 0.0);
    }
    if let Some(hr) = world.hit(r, 0.001, INFINITY) {
        let target: Vec3A = hr.p + hr.norm + random_unit_vector(rng);
        0.5 * ray_color(&Ray::new(hr.p, target - hr.p), world, depth - 1, rng)
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3A::new(1.0, 1.0, 1.0) + t * Vec3A::new(0.5, 0.7, 1.0)
    }
}

fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3A {
    let mut cont: bool = true;
    let mut p: Vec3A = Vec3A::new(0.0, 0.0, 0.0);
    while cont {
        p = Vec3A::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if p.length_squared() >= 1.0 {
            cont = true;
        } else {
            cont = false;
        }
    }
    return p;
}
fn random_unit_vector<R: Rng>(rng: &mut R) -> Vec3A {
    random_in_unit_sphere(rng).normalize()
}
