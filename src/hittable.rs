use crate::ray::Ray;
use glam::Vec3A;
type Point3 = Vec3A;
pub struct HitRecord {
    pub p: Point3,
    pub norm: Vec3A,
    pub t: f32,
}
pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList<'a> {
    list: Vec<Box<Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList { list: Vec::new() }
    }
    pub fn add_hittable<T: Hittable + 'a>(&mut self, h: T) {
        self.list.push(Box::new(h));
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for h in &self.list {
            if let Some(hr) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hr.t;
                result = Some(hr);
            }
        }
        return result;
    }
}
