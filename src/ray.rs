use glam::Vec3A;
type Point3 = Vec3A;
pub struct Ray {
    origin: Point3,
    direction: Vec3A,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3A) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3A {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}
