use crate::ray::Ray;
use glam::Vec3A;
type Point3 = Vec3A;
pub struct HitRecord {
    p: Point3,
    norm: Vec3A,
    t: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
