use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use glam::Vec3A;
type Point3 = Vec3A;

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3A = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - a * c;
        if disc >= 0.0 {
            let sqrt_disc = disc.sqrt();
            let t = (-half_b - sqrt_disc) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let norm = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, norm });
            }
            let t = (-half_b + sqrt_disc) / a;
            if t < t_max && t > t_min {
                let p = ray.at(t);
                let norm = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, norm });
            }
        }
        None
    }
}
