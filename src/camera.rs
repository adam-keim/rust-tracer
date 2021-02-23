use glam::Vec3A;

use crate::ray::Ray;
type Point3 = Vec3A;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3A,
    vertical: Vec3A,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3A::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3A::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3A::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (v * self.vertical) - self.origin,
        );
    }
}
